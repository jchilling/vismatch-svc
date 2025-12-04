use std::cmp::Ordering;
use std::error::Error;

use imagehash::Hash;
use serde;
use image;
use std::fs::{File, read_dir};
use std::path::{Path, PathBuf};
use vismatch_svc::metric::*;

// Some common ext for images.
const IMAGE_EXTENSIONS: [&str; 8] = [
    "png", "jpg", "jpeg", "gif", "bmp", "ico", "webp", "tiff" // We could consider accept top-3 later?
];

#[derive(serde::Serialize, serde::Deserialize)]
pub struct HashProxy {
    /// The bit vector representation of the hash.
    pub bits: Vec<bool>,
}

#[derive(Debug, Clone, Copy)]
pub enum HashType {
    DHASH,
    PHASH,
}

/// Write hash value to cache file in the same folder
/// of image file located.
fn write_hash_cache(img_path: &Path, img_hash: &Hash, hash_type: HashType) -> Result<usize, Box<dyn Error>> {

    let img_path = img_path.to_owned();

    let hash_file_name = match hash_type {
        HashType::DHASH => img_path.with_added_extension("dhash"),
        HashType::PHASH => img_path.with_added_extension("phash"),
    };

    // Serialize: using proxy trick.
    let hash_pxy = 
        HashProxy { bits: img_hash.bits.clone() }; // clone to a already-derived (de)serialize struct.

    let mut f_handle = File::create(hash_file_name)?;

    bincode::serde::encode_into_std_write(
                            &hash_pxy,
                            &mut f_handle,
                            bincode::config::standard())
                                    .map_err(|e| format!("error while serialize ({})", e).into())
}


/// Load hash value from cache in the folder of given image.
/// 
pub fn fetch_hash_cache(img_path: &Path, hash_type: HashType) -> Result<Hash, Box<dyn Error>> {
    
    let hash_file_name = match hash_type {
        HashType::DHASH => img_path.with_added_extension("dhash"),
        HashType::PHASH => img_path.with_added_extension("phash"),
    };

    // try to open the cache corresponding to the given hash type
    let mut f_handle = match File::open(&hash_file_name) {
        Ok(f) => f,
        Err(e) => {
            // Provide a more descriptive error if the file doesn't exist
            return Err(format!("cannot open cache file '{}' with type {:?}: {}",
                                hash_file_name.display(), hash_type, e).into());
        }
    };

    // try to decode
    let hash_pxy: HashProxy = 
        bincode::serde::decode_from_std_read(
        &mut f_handle,
        bincode::config::standard(),
        ).map_err(|e: bincode::error::DecodeError| format!("cannot deserialize cache file '{}' with type {:?}: {}",
                            hash_file_name.display(), hash_type, e))?;

    let img_hash = Hash {
        bits: hash_pxy.bits.clone(),
    };

    Ok(img_hash)
}

#[derive(Debug)]
struct HashListEntry {
    image_name: PathBuf,
    hash: Hash,
}

#[derive(Debug)]
struct HashDistEntry {
    image_name: PathBuf,
    distance: f64,
}

impl PartialEq for HashDistEntry {
    fn eq(&self, other: &Self) -> bool {
        // Equality is defined by the total comparison being equal.
        self.distance.total_cmp(&other.distance) == Ordering::Equal
    }
}

impl Eq for HashDistEntry {}

impl PartialOrd for HashDistEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // You can use standard partial_cmp here, though total_cmp is also fine.
        self.distance.partial_cmp(&other.distance)
    }
}

impl Ord for HashDistEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        // Use total_cmp to get a stable, panic-free total ordering.
        self.distance.total_cmp(&other.distance)
    }
}

fn calculate_similarity(img_path: &Path, hash_list: Vec<HashListEntry>) -> Vec<HashDistEntry> {
    // first, we load the image
    let img = image::open(img_path).expect("cannot load test image"); // catch error in real implementation

    // choose proper hash algorithm in actual impl
    let hasher = imagehash::PerceptualHash::new()
    .with_image_size(32, 32)
    .with_hash_size(32, 32)
    .with_resizer(|img, w, h| {
        // Your custom resizer function
        img.resize_exact(w as u32, h as u32, image::imageops::FilterType::Lanczos3)
    });

    let h = hasher.hash(&img);

    hash_list.iter().map(|h_ent| {
        let h_dist = h.dist(&h_ent.hash);

        HashDistEntry {
            image_name: h_ent.image_name.clone(),
            distance: h_dist,    
        }
    }).collect()

}

fn main() {

    // Setup hasher
    let p_hasher = imagehash::PerceptualHash::new()
    .with_image_size(32, 32)
    .with_hash_size(32, 32)
    .with_resizer(|img, w, h| {
        // Your custom resizer function
        img.resize_exact(w as u32, h as u32, image::imageops::FilterType::Lanczos3)
    });

    let d_hasher = imagehash::DifferenceHash::new()
    .with_image_size(32, 32)
    .with_hash_size(32, 32)
    .with_resizer(|img, w, h| {
        // Your custom resizer function
        img.resize_exact(w as u32, h as u32, image::imageops::FilterType::Lanczos3)
    });

    let mut hash_entries : Vec<HashListEntry> = Vec::new();

    let root_dir = Path::new("./resources/val2017");

    let dir_entries = read_dir(root_dir)
                                    .expect("cannot read folder contents");

    // load all dataset and calculate hash
    for ent in dir_entries {
        let ent = match ent {
            Ok(ent) => ent,
            Err(e) => {
                println!("Error encounted while reading content: {}", e);
                continue;
            }
        };

        let img_path = ent.path();
        
        // Skip directories and non-files
        if !img_path.is_file() {
            continue;
        }

        // Check image extencsion
        let is_image = img_path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| IMAGE_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
            .unwrap_or(false);

        if is_image {
            // try to load cache first
            match fetch_hash_cache(&img_path, HashType::PHASH) { // We use phash for this experiement
                Ok(h) => {
                    hash_entries.push(HashListEntry { image_name: img_path.to_owned(), hash: h });
                    continue;
                },
                Err(_e) => {
                    println!("new image found, calculating new hash cache: {:?}", img_path);
                },
            }

            match image::open(img_path.clone()) {
                Err(e) => println!("cannot open img <{}> due to: {}", img_path.display(), e),
                Ok(img) => {
                    let p_hash = p_hasher.hash(&img);
                    let d_hash = d_hasher.hash(&img);
                    println!("{} => {:?}", img_path.display(), p_hash);

                    // Serialize to cache file
                    write_hash_cache(&img_path, &p_hash, HashType::PHASH)
                        .map_err(|err| println!("error while serializing to file: {}", err))
                        .ok();
                    
                    write_hash_cache(&img_path, &d_hash, HashType::DHASH)
                        .map_err(|err| println!("error while serializing to file: {}", err))
                        .ok();


                    println!("Successfully calculated hash for image: {:?}", img_path);
                    hash_entries.push(HashListEntry { image_name: img_path.to_owned(), hash: p_hash }); // we use phash only for this time
                }
            }
        }
    }

    let test_img_path = PathBuf::from("./test4.jpg");

    let mut compared_res = calculate_similarity(&test_img_path, hash_entries);
    compared_res.sort(); // inplace sort

    let top3 = &compared_res[0..3];

    println!("===Top 3 results===\n");

    top3.iter().for_each(|elem| {
        println!("image name: {:?} | score: {}", elem.image_name, elem.distance);
    });

}


