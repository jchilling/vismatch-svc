//! Storage abstraction for images and metadata.
//! Supports both local filesystem and Google Cloud Storage (Firebase Storage).

use async_trait::async_trait;
use std::error::Error;
use std::path::Path;
use image::DynamicImage;
use std::io::Cursor;

#[async_trait]
pub trait StorageBackend: Send + Sync {
    /// Save an image to storage
    async fn save_image(
        &self,
        project_name: &str,
        image_name: &str,
        image: &DynamicImage,
    ) -> Result<String, Box<dyn Error + Send + Sync>>;

    /// Load an image from storage
    async fn load_image(
        &self,
        project_name: &str,
        image_name: &str,
    ) -> Result<DynamicImage, Box<dyn Error + Send + Sync>>;

    /// Delete an image from storage
    async fn delete_image(
        &self,
        project_name: &str,
        image_name: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;

    /// Delete all images in a project
    async fn delete_project(
        &self,
        project_name: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;

    /// List all images in a project
    async fn list_images(
        &self,
        project_name: &str,
    ) -> Result<Vec<String>, Box<dyn Error + Send + Sync>>;
}

/// Local filesystem storage (for development/local deployment)
pub struct LocalStorage {
    project_root: String,
}

impl LocalStorage {
    pub fn new(project_root: String) -> Self {
        Self { project_root }
    }
}

#[async_trait]
impl StorageBackend for LocalStorage {
    async fn save_image(
        &self,
        project_name: &str,
        image_name: &str,
        image: &DynamicImage,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        use std::fs::create_dir;
        use std::path::PathBuf;

        let project_path = PathBuf::from(&self.project_root).join(project_name);
        
        // Create project directory if it doesn't exist
        if !project_path.exists() {
            create_dir(&project_path)?;
        }

        let image_path = project_path.join(image_name);
        image.save(&image_path)?;
        
        Ok(image_path.to_string_lossy().to_string())
    }

    async fn load_image(
        &self,
        project_name: &str,
        image_name: &str,
    ) -> Result<DynamicImage, Box<dyn Error + Send + Sync>> {
        use std::path::PathBuf;
        let image_path = PathBuf::from(&self.project_root)
            .join(project_name)
            .join(image_name);
        
        let img = image::open(&image_path)?;
        Ok(img)
    }

    async fn delete_image(
        &self,
        project_name: &str,
        image_name: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        use std::fs::remove_file;
        use std::path::PathBuf;
        
        let image_path = PathBuf::from(&self.project_root)
            .join(project_name)
            .join(image_name);
        
        remove_file(&image_path)?;
        Ok(())
    }

    async fn delete_project(
        &self,
        project_name: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        use std::fs::remove_dir_all;
        use std::path::PathBuf;
        
        let project_path = PathBuf::from(&self.project_root).join(project_name);
        remove_dir_all(&project_path)?;
        Ok(())
    }

    async fn list_images(
        &self,
        project_name: &str,
    ) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
        use std::fs::read_dir;
        use std::path::PathBuf;
        use crate::utils::is_image_file;
        
        let project_path = PathBuf::from(&self.project_root).join(project_name);
        
        if !project_path.exists() {
            return Ok(Vec::new());
        }

        let entries = read_dir(&project_path)?;
        let mut images = Vec::new();

        for entry in entries {
            let entry = entry?;
            if is_image_file(&entry) {
                if let Some(name) = entry.file_name().to_str() {
                    images.push(name.to_string());
                }
            }
        }

        Ok(images)
    }
}

/// Google Cloud Storage backend (for production/Firebase)
/// Uses GCS REST API via HTTP
pub struct GcsStorage {
    bucket_name: String,
}

impl GcsStorage {
    pub fn new(bucket_name: String) -> Self {
        Self { bucket_name }
    }

    fn image_path(&self, project_name: &str, image_name: &str) -> String {
        format!("{}/{}", project_name, image_name)
    }

    async fn get_access_token(&self) -> Result<String, Box<dyn Error + Send + Sync>> {
        // Use Application Default Credentials (ADC)
        // When running on Cloud Run, this is automatically available
        // For local development, use: gcloud auth application-default login
        
        use std::process::Command;
        
        // Try to get token from gcloud CLI (for local dev)
        let output = Command::new("gcloud")
            .args(&["auth", "print-access-token"])
            .output();
        
        match output {
            Ok(output) if output.status.success() => {
                let token = String::from_utf8(output.stdout)?;
                Ok(token.trim().to_string())
            }
            _ => {
                // In Cloud Run, use metadata server
                let client = reqwest::Client::new();
                let token_url = "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token";
                
                let response = client
                    .get(token_url)
                    .header("Metadata-Flavor", "Google")
                    .send()
                    .await?;
                
                let json: serde_json::Value = response.json().await?;
                Ok(json["access_token"].as_str().unwrap_or("").to_string())
            }
        }
    }
}

#[async_trait]
impl StorageBackend for GcsStorage {
    async fn save_image(
        &self,
        project_name: &str,
        image_name: &str,
        image: &DynamicImage,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        // Convert image to bytes
        let mut image_data = Vec::new();
        image.write_to(
            &mut Cursor::new(&mut image_data),
            image::ImageOutputFormat::Png,
        )?;

        let object_name = self.image_path(project_name, image_name);
        let access_token = self.get_access_token().await?;
        
        // Upload to GCS using REST API
        let upload_url = format!(
            "https://storage.googleapis.com/upload/storage/v1/b/{}/o?uploadType=media&name={}",
            self.bucket_name,
            urlencoding::encode(&object_name)
        );

        let client = reqwest::Client::new();
        let response = client
            .post(&upload_url)
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Content-Type", "image/png")
            .body(image_data)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("GCS upload failed: {}", error_text).into());
        }

        Ok(format!("gs://{}/{}", self.bucket_name, object_name))
    }

    async fn load_image(
        &self,
        project_name: &str,
        image_name: &str,
    ) -> Result<DynamicImage, Box<dyn Error + Send + Sync>> {
        let object_name = self.image_path(project_name, image_name);
        let access_token = self.get_access_token().await?;

        // Download from GCS using REST API
        let download_url = format!(
            "https://storage.googleapis.com/{}/{}",
            self.bucket_name,
            urlencoding::encode(&object_name)
        );

        let client = reqwest::Client::new();
        let response = client
            .get(&download_url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("GCS download failed: {}", response.status()).into());
        }

        let data = response.bytes().await?;
        let img = image::load_from_memory(&data)?;
        Ok(img)
    }

    async fn delete_image(
        &self,
        project_name: &str,
        image_name: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let object_name = self.image_path(project_name, image_name);
        let access_token = self.get_access_token().await?;

        let delete_url = format!(
            "https://storage.googleapis.com/storage/v1/b/{}/o/{}",
            self.bucket_name,
            urlencoding::encode(&object_name)
        );

        let client = reqwest::Client::new();
        let response = client
            .delete(&delete_url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;

        if !response.status().is_success() && response.status() != reqwest::StatusCode::NOT_FOUND {
            return Err(format!("GCS delete failed: {}", response.status()).into());
        }

        Ok(())
    }

    async fn delete_project(
        &self,
        project_name: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // List all objects with the project prefix
        let prefix = format!("{}/", project_name);
        let access_token = self.get_access_token().await?;

        let list_url = format!(
            "https://storage.googleapis.com/storage/v1/b/{}/o?prefix={}",
            self.bucket_name,
            urlencoding::encode(&prefix)
        );

        let client = reqwest::Client::new();
        let response = client
            .get(&list_url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("GCS list failed: {}", response.status()).into());
        }

        let json: serde_json::Value = response.json().await?;
        
        if let Some(items) = json["items"].as_array() {
            for item in items {
                if let Some(name) = item["name"].as_str() {
                    let delete_url = format!(
                        "https://storage.googleapis.com/storage/v1/b/{}/o/{}",
                        self.bucket_name,
                        urlencoding::encode(name)
                    );

                    let _ = client
                        .delete(&delete_url)
                        .header("Authorization", format!("Bearer {}", access_token))
                        .send()
                        .await;
                }
            }
        }

        Ok(())
    }

    async fn list_images(
        &self,
        project_name: &str,
    ) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
        let prefix = format!("{}/", project_name);
        let access_token = self.get_access_token().await?;

        let list_url = format!(
            "https://storage.googleapis.com/storage/v1/b/{}/o?prefix={}",
            self.bucket_name,
            urlencoding::encode(&prefix)
        );

        let client = reqwest::Client::new();
        let response = client
            .get(&list_url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("GCS list failed: {}", response.status()).into());
        }

        let json: serde_json::Value = response.json().await?;
        let mut images = Vec::new();

        if let Some(items) = json["items"].as_array() {
            for item in items {
                if let Some(name) = item["name"].as_str() {
                    // Extract just the filename (remove project_name/ prefix)
                    if let Some(filename) = name.strip_prefix(&format!("{}/", project_name)) {
                        images.push(filename.to_string());
                    }
                }
            }
        }

        Ok(images)
    }
}

/// Create storage backend based on environment variables
pub fn create_storage_backend() -> Result<Box<dyn StorageBackend>, Box<dyn Error + Send + Sync>> {
    // Check for GCS bucket name
    if let Ok(bucket_name) = std::env::var("GCS_BUCKET_NAME") {
        println!("[*] Using Google Cloud Storage: {}", bucket_name);
        Ok(Box::new(GcsStorage::new(bucket_name)))
    } else {
        // Default to local storage
        let project_root = std::env::var("PROJECT_ROOT")
            .unwrap_or_else(|_| "./image_root".to_string());
        println!("[*] Using local filesystem storage: {}", project_root);
        Ok(Box::new(LocalStorage::new(project_root)))
    }
}

