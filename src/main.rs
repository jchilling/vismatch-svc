
use regex::Regex;
use std::cmp::min;
use std::error::Error;          // standard error trait
use std::time::Instant;         // calculate time difference
use std::collections::HashMap;  // hashmap support
use image::DynamicImage;        // image IO
use itertools::Itertools;       // functional pattern support to make life easier

// asynchronous execution and management
use tokio::sync::RwLock;    // shared object management
use std::sync::Arc;         // shared object reference

// HTTP related libs
use axum::http::{Response, StatusCode, Method, HeaderMap}; // HTTP
use axum::response::IntoResponse;       // convert to response
use axum::routing::{post, delete};     // HTTP methods
use axum::body::Body;                   // plain response body
use axum::extract::{Json, State, Path as PathParam, Request}; // response types
use axum::middleware::Next;
use axum::{Router, http};               // router
use tokio::net::TcpListener;            // listener
use std::net::SocketAddr;               // socker definition
use tower_http::cors::{CorsLayer, Any}; // CORS support

// filesystem and os-related libraries
use std::path::{Path, PathBuf, Component};      // filesystem path operations
use std::fs::{read_dir, create_dir}; // filesystem utils
use std::io::Write; // file writing

// internal libraries
use vismatch_svc::{
    HasSingleImage,         // trait for getting image from request object
    base64_to_image, 
    dist_entry_to_api_sim_entry_with_storage, image_hash::*};     // our packaged hash algorithms

use vismatch_svc::project_mgmt::{
    load_or_calc_project_hashes     
};
use vismatch_svc::api::*;           // API structure
use vismatch_svc::storage::{StorageBackend, create_storage_backend, load_all_project_hashes_from_gcs_env};
use vismatch_svc::auth::{verify_iap_auth, IapUser};


type ProjectHashDict = Arc<RwLock<HashMap<String, Vec<ImageHashEntry>>>>;

#[derive(Clone)]
struct AppState {
    storage: Arc<dyn StorageBackend>,
    project_dict: ProjectHashDict,
}

// common task definition

fn validate_project_name(name: &str) -> Result<(), AppError> {
    let name = name.trim();

    if name.is_empty() {
        return Err(AppError::BadRequest("project_name cannot be empty".into()));
    }
    if name.len() > 64 {
        return Err(AppError::BadRequest("project_name too long".into()));
    }

    // 必須是「單一 segment」：不能包含 /，不能是 . 或 ..，不能是絕對路徑
    let mut comps = Path::new(name).components();
    match (comps.next(), comps.next()) {
        (Some(Component::Normal(_)), None) => {}
        _ => return Err(AppError::BadRequest("invalid project_name path".into())),
    }

    // 字元白名單：中文(漢字)+英數+底線
    // 若你也想允許 -，改成 [\\p{Han}A-Za-z0-9_-]+
    let re = Regex::new(r"^[\p{Han}A-Za-z0-9_]+$").unwrap();
    if !re.is_match(name) {
        return Err(AppError::BadRequest(
            "project_name only allows Chinese characters, letters, numbers, and '_'".into(),
        ));
    }

    Ok(())
}
async fn save_image_to_project(
    storage: Arc<dyn StorageBackend>,
    project_name: &str, 
    image: &DynamicImage, 
    image_name: &str,
    hash_type: HashType,
    project_hashes: ProjectHashDict) -> Result<(), Box<dyn Error + Send + Sync>> {

    let _project_hashes = Arc::clone(&project_hashes);
    let mut project_dict_wlock = _project_hashes.write().await;

    // Check if project exists in dict, if not create entry
    if !(*project_dict_wlock).contains_key(project_name) {
        (*project_dict_wlock).insert(project_name.to_owned(), Vec::<ImageHashEntry>::new());
    }

    // Save the image using storage backend
    println!("[*] saving image to storage: {}/{}", project_name, image_name);
    let _storage_path = storage.save_image(project_name, image_name, image).await?;
    println!("[*] image saved to: {}", _storage_path);

    // Calculate hash directly from the image (we already have it in memory)
    let image_clone = image.clone();
    let project_name_clone = project_name.to_string();
    let image_name_clone = image_name.to_string();
    let hash_calc_task = 
        tokio::task::spawn_blocking(move || -> Result<ImageHashEntry, Box<dyn Error + Send + Sync>> {
            use vismatch_svc::image_hash::{mk_hasher, Hash};
            use std::path::PathBuf;
            
            let hasher = mk_hasher(hash_type);
            let hash: Hash = hasher.hash(&image_clone).into();
            
            // Create a virtual path for the hash entry (for compatibility)
            let virtual_path = PathBuf::from(&project_name_clone).join(&image_name_clone);
            
            Ok(ImageHashEntry {
                image_name: virtual_path,
                hash_type,
                hash,
            })
        });

    let hash_result: ImageHashEntry = hash_calc_task.await??;

    // Update the project hash dict
    if let Some(val) = (*project_dict_wlock).get_mut(project_name) { 
        val.push(hash_result); 
    }

    Ok(()) // All good, return
}


/// For a given image and specified project name, calculate
/// the difference list across project images for provided image.
async fn calc_sim_in_project(image: DynamicImage, project_name: &str, project_hashes: ProjectHashDict) 
    -> Result<Vec<ImageDistEntry>, Box<dyn Error + Send + Sync>>{
    // println!("[*] enter calculation blk");

    let calc_start = Instant::now(); // Measure calc time

    let image = image.clone();
    let project_dict_rlock = project_hashes.read().await;

    // first, we should check if the project exists.
    match (*project_dict_rlock).get(project_name) {

        // If exists, then calculate the distance.
        Some(hash_list) => {
            let hash_list = hash_list.clone();

            // This involves image resizing, which is a cpu task.
            // So we put it in seprated thread. 
            let diff_calc_task = 
                tokio::task::spawn_blocking(move || {            
                    let res = calc_similarity_list(&image, &hash_list);  
                    res
                });

            let mut diff_result = diff_calc_task.await?;
            diff_result.sort();

            let calc_done = calc_start.elapsed(); // Measure load time

            println!("[*] calculation task done: {:.3?}", calc_done);
            // println!("[*] leave calculation blk");
            
            Ok(diff_result)

        },
        None => Err(format!("project <{}> not found in current database", project_name).into()),
    }
}

// here's are the service handlers

async fn compare_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<CompareImageReq>)
    -> Result<Json<CompareImageResp>, AppError> {
    
    // Verify IAP authentication
    let _iap_user = verify_iap_auth(&headers).await
        .map_err(|_| AppError::Unauthorized("IAP authentication required".into()))?;
    
    // 1. we first get the image from data b64 string
    let image_target 
        = payload.get_image()
            .map_err(|e| AppError::InternalError(e.to_string()))?;

    // 2. 
    let result = calc_sim_in_project(
        image_target, 
        &payload.project_name, 
        state.project_dict
    ).await.map_err(|e| AppError::BadRequest(e.to_string()));

    match result {
        Ok(dist_vec) => {

            // [NOTE] we pick the top-3 entries from closest images, change if needed.
            let ending_index = min(dist_vec.len(), 3);
            let storage = Arc::clone(&state.storage);
            let project_name = payload.project_name.clone();
            
            let mut sim_vec = Vec::new();
            for entry in &dist_vec[0..ending_index] {
                let sim_entry = dist_entry_to_api_sim_entry_with_storage(
                    entry,
                    payload.with_image,
                    storage.as_ref(),
                    &project_name,
                ).await;
                sim_vec.push(sim_entry);
            }
            
            Ok(Json(CompareImageResp {
            success: true,
            message: "success".to_owned(),
            project_name: payload.project_name,
            compare_result: sim_vec,
        }))},
        Err(e) => Err(e),
    }
}

async fn upload_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<UploadImageReq>)
    -> Result<Json<UploadImageResp>, AppError> {
    
    // Verify IAP authentication
    let _iap_user = verify_iap_auth(&headers).await
        .map_err(|_| AppError::Unauthorized("IAP authentication required".into()))?;
    
    // 1. we first collect parameters we need

    let project_name = payload.project_name;
    let image_name = payload.image_name;
    
    // Validate project name to prevent path traversal attacks
    validate_project_name(&project_name)?;

    // Check for duplicate image name within the same project
    let storage = Arc::clone(&state.storage);
    let existing_images = storage
        .list_images(&project_name)
        .await
        .map_err(|e| AppError::InternalError(format!("failed to list project images: {}", e)))?;

    if existing_images.iter().any(|name| name == &image_name) {
        return Ok(Json(UploadImageResp {
            success: false,
            message: format!(
                "image '{}' already exists in project '{}', please use a different name",
                image_name, project_name
            ),
            token: "".to_string(),
        }));
    }

    // [NOTE] consider resize to save spaces.
    let image = base64_to_image(&payload.data)
                .map_err(|e| format!("cannot create image from b64: {}", e.to_string()))
                .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let project_dict = Arc::clone(&state.project_dict);

    println!("[*] received upload request on <{}>", project_name); // [NOTE] verbose

    // do saving image, return 500 if failed
    save_image_to_project(
        storage,
        &project_name,
        &image,
        &image_name,
        HashType::PHASH, // [NOTE] [WARN] change here later
        project_dict
    ).await.map_err(|e| AppError::InternalError(e.to_string()))?;

    Ok(Json(UploadImageResp {
        success: true,
        message: "image uploaded and indexed successfully".to_owned(),
        token: "dummy-deletion-token".to_string(), // [WARN] [NOTE] change later to proper uuid
    }))

}

async fn delete_project_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    PathParam(project_name): PathParam<String>)
    -> Result<Json<DeleteProjectResp>, AppError> {
    
    // Verify IAP authentication
    let _iap_user = verify_iap_auth(&headers).await
        .map_err(|_| AppError::Unauthorized("IAP authentication required".into()))?;
        
        println!("[*] received delete project request for: <{}>", project_name);
        validate_project_name(&project_name)?;
        let project_dict = Arc::clone(&state.project_dict);
        let storage = Arc::clone(&state.storage);

        // Check if project exists in dict
        let project_exists = {
            let project_dict_rlock = project_dict.read().await;
            project_dict_rlock.contains_key(&project_name)
        };

        if !project_exists {
            return Ok(Json(DeleteProjectResp {
                success: false,
                message: format!("Project '{}' does not exist", project_name),
            }));
        }

        // Remove from in-memory hash dict first
        {
            let mut project_dict_wlock = project_dict.write().await;
            project_dict_wlock.remove(&project_name);
        }

        // Delete the project from storage
        match storage.delete_project(&project_name).await {
            Ok(_) => {
                println!("[*] deleted project <{}>", project_name);
                Ok(Json(DeleteProjectResp {
                    success: true,
                    message: format!("Project '{}' deleted successfully", project_name),
                }))
            },
            Err(e) => {
                Err(AppError::InternalError(format!("Failed to delete project: {}", e)))
            }
        }
    }


/// Handler for "404 not found" error, returning plain text body.
async fn not_found_handler() -> Response<Body> { 
    let response = Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header(http::header::CONTENT_TYPE, "application/json")
        .header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .header(http::header::ACCESS_CONTROL_ALLOW_METHODS, "GET, POST, DELETE, OPTIONS")
        .header(http::header::ACCESS_CONTROL_ALLOW_HEADERS, "*")
        .body(Body::from("Knock, knock. Anyone here?\n\nSorry, this door seems to be missing! Maybe try another link?".to_owned()))
        .unwrap();
    
    response.into_response()
}

#[tokio::main]
async fn main() {
    eprintln!("[*] Starting vismatch-svc...");
    eprintln!("[*] Environment check:");
    eprintln!("  - GCS_BUCKET_NAME: {:?}", std::env::var("GCS_BUCKET_NAME").ok());
    eprintln!("  - PORT: {:?}", std::env::var("PORT").ok());

    // Stage 1: Initialize storage backend
    eprintln!("[*] Stage 1: Initializing storage backend...");
    let storage = Arc::from(create_storage_backend()
        .map_err(|e| format!("Failed to initialize storage: {}", e))
        .unwrap_or_else(|e| {
            eprintln!("[x] Failed to initialize storage: {}", e);
            eprintln!("[x] Shutting down.");
            std::process::exit(1);
        }));

    // Stage 2: Load projects from storage (GCS or local filesystem)
    eprintln!("[*] Stage 2: Loading projects from storage...");
    let standard_hash_type: HashType = HashType::PHASH;
    let load_all = Instant::now();

    let (project_name_hash_map, needs_background_indexing): (ProjectHashDict, bool) = if std::env::var("GCS_BUCKET_NAME").is_ok() {
        // GCS storage: start with empty index, rebuild in background to avoid startup timeout
        println!("[*] Detected GCS_BUCKET_NAME, starting with empty index (will rebuild in background)...");
        (Arc::new(RwLock::new(HashMap::new())), true)
    } else {
        // Local storage: load from filesystem
        let project_root: &Path = Path::new("./image_root");

        let is_project_root_exists = 
            project_root.try_exists()
                    .expect("[x] can't check existence of project root folder, shutting down.");

        match is_project_root_exists {
            false => {
                match create_dir(project_root) {
                    Ok(_) => println!("[*] created project root folder."),
                    Err(_) => panic!("[x] cannot create project folder, shutting down."),
                }
            },
            true => {
                match project_root.is_dir() {
                    false => panic!("[x] project folder is not valid, shutting down."),
                    true => {}, // Do nothing, continue the service process
                }
            }
        }

        let child_project_reader = 
            read_dir(project_root)
                .map_err(|e: std::io::Error| format!("error reading root project contents: <{}>", e))
                .unwrap();

        let (children_projects, _): (Vec<_>, Vec<_>) = 
            child_project_reader.filter_ok(|f| f.path().is_dir())
                    .map_ok(|f| f.path())
                    .partition_result();

        let (children_project_hashes, _): 
            (Vec<(String, Vec<ImageHashEntry>)>, Vec<_>) = 
                children_projects.into_iter()
                    .map(|f: PathBuf| {
                        match load_or_calc_project_hashes(&f, standard_hash_type) {
                            Ok(h) => {
                                let project_name = 
                                    f.file_name().ok_or("invalid project name")?;
                                Ok((project_name.to_string_lossy().into_owned(), h))
                            },
                            Err(err) => Err(err),
                        }})
                    .partition_result();

        (Arc::new(RwLock::new(children_project_hashes.into_iter().collect())), false)
    };

    let load_all_done = load_all.elapsed(); // Measure load time

    // [NOTE] any other init stage thingy goes here.

    println!("[*] initialization stage costs: {:.3?}", load_all_done);
    println!("[v] initialization stage done, starting service...");

    // Read port from PORT env var (Cloud Run sets this), default to 3000
    let port_env = std::env::var("PORT").ok();
    let port: u16 = port_env
        .as_ref()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);
    
    eprintln!("[*] PORT env var: {:?}, using port: {}", port_env, port);

    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], port));
    eprintln!("[*] Attempting to bind to address: {}", addr);

    let listener = match TcpListener::bind(addr).await {
        Ok(l) => {
            eprintln!("[*] Successfully bound to {}", addr);
            l
        },
        Err(e) => {
            eprintln!("[x] Failed to bind to {}: {}", addr, e);
            eprintln!("[x] Shutting down.");
            std::process::exit(1);
        }
    };

    println!("[*] image comparison service listening on {}", addr);


    // Stage 3: starting service
    eprintln!("[*] Stage 3: Setting up Axum router...");
    eprintln!("[*] IAP authentication: ENABLED (expecting X-Goog-Authenticated-User-* headers)");
    
    let axum_state: AppState = AppState { 
        storage: Arc::clone(&storage),
        project_dict: Arc::clone(&project_name_hash_map),
    };

    // Configure CORS to allow requests from frontend
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::OPTIONS])
        .allow_headers(Any)
        .expose_headers(Any);
    
    let axum_app: Router = Router::new()
                    .route("/diff", post(compare_handler))
                    .route("/upload", post(upload_handler))
                    .route("/project/{project_name}", delete(delete_project_handler))
                    .fallback(not_found_handler)
                    .with_state(axum_state)
                    .layer(cors);

    // Spawn background task to rebuild GCS index if needed
    if needs_background_indexing {
        let bg_storage = Arc::clone(&storage);
        let bg_dict = Arc::clone(&project_name_hash_map);
        tokio::spawn(async move {
            println!("[*] Starting background GCS index rebuild...");
            // #region agent log
            let bg_log_path = "/Users/chy/Desktop/vismatch-svc/.cursor/debug.log";
            let mut bg_log_file = std::fs::OpenOptions::new().create(true).append(true).open(bg_log_path).ok();
            let bg_start = Instant::now();
            if let Some(ref mut file) = bg_log_file {
                let _ = writeln!(file, "{{\"sessionId\":\"debug-session\",\"runId\":\"background\",\"hypothesisId\":\"C\",\"location\":\"main.rs:bg\",\"message\":\"background GCS indexing start\",\"data\":{{}},\"timestamp\":{}}}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis());
            }
            // #endregion
            match load_all_project_hashes_from_gcs_env(standard_hash_type).await {
                Ok(children_project_hashes) => {
                    let mut dict = bg_dict.write().await;
                    *dict = children_project_hashes.into_iter().collect();
                    let bg_duration = bg_start.elapsed();
                    // #region agent log
                    if let Some(ref mut file) = bg_log_file {
                        let _ = writeln!(file, "{{\"sessionId\":\"debug-session\",\"runId\":\"background\",\"hypothesisId\":\"C\",\"location\":\"main.rs:bg\",\"message\":\"background GCS indexing complete\",\"data\":{{\"duration_sec\":{},\"project_count\":{}}},\"timestamp\":{}}}", bg_duration.as_secs_f64(), dict.len(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis());
                    }
                    // #endregion
                    println!("[*] Background GCS index rebuild complete in {:.3?}", bg_duration);
                },
                Err(e) => {
                    // #region agent log
                    if let Some(ref mut file) = bg_log_file {
                        let _ = writeln!(file, "{{\"sessionId\":\"debug-session\",\"runId\":\"background\",\"hypothesisId\":\"D\",\"location\":\"main.rs:bg\",\"message\":\"background GCS indexing error\",\"data\":{{\"error\":\"{}\"}},\"timestamp\":{}}}", e, std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis());
                    }
                    // #endregion
                    eprintln!("[x] Background GCS index rebuild failed: {}", e);
                }
            }
        });
    }

    eprintln!("[*] Starting Axum server...");
    match axum::serve(listener, axum_app).await {
        Ok(_) => {
            eprintln!("[*] Server stopped normally");
        },
        Err(e) => {
            eprintln!("[x] Server error: {}", e);
            std::process::exit(1);
        }
    }
}


