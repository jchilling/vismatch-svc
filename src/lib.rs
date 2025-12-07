pub mod api;
pub mod vec_ops;
pub mod metric;
pub mod image_hash;
pub mod project_mgmt;
mod utils;

pub use utils::is_image_file;


use api::*;
use image::DynamicImage;

use crate::image_hash::ImageDistEntry;

pub fn base64_to_image(base64_str: &str) 
    -> Result<image::DynamicImage, Box<dyn std::error::Error>> {
    
    use base64::{engine::general_purpose, Engine};
    
    let data_b64: String = base64_str.to_owned();

    // Extract the raw Base64 content (if a data URI is present)
    let raw_base64_content: &str = if data_b64.starts_with("data:") {
        let parts: Vec<&str> = data_b64.split(',').collect();
        if parts.len() < 2 {
            return Err("found data URI format, but not valid".into());
        }
        parts[1].trim()
    } else {
        data_b64.as_str()
    };

    let decoded_bytes = general_purpose::STANDARD.decode(raw_base64_content)?;

    let img_decoded = image::load_from_memory(&decoded_bytes)?;

    Ok(img_decoded)
}

pub fn image_to_base64(image: &DynamicImage) 
    -> Result<String, Box<dyn std::error::Error>> {

    use base64::{engine::general_purpose, Engine};
    use std::io::Cursor;

    let mut image_data: Vec<u8> = Vec::new();

    image.write_to(
        &mut Cursor::new(&mut image_data), 
        image::ImageOutputFormat::Png)
            .map_err(|_e| format!("base64 encode error: cannot write to intermediate buffer"))?;

    let b64_str = general_purpose::STANDARD.encode(image_data);

    Ok(b64_str)
}

/// Indicates that a request (or payload) has at least
/// one single image.
pub trait HasSingleImage {
    fn get_image(&self) -> Result<image::DynamicImage, Box<dyn std::error::Error>>;
}

impl HasSingleImage for UploadImageReq {
    fn get_image(&self) -> Result<image::DynamicImage, Box<dyn std::error::Error>> {
        base64_to_image(&self.data)
    }
}

impl HasSingleImage for CompareImageReq {
    fn get_image(&self) -> Result<image::DynamicImage, Box<dyn std::error::Error>> {
        base64_to_image(&self.data)
    }
}

/// Convert a`ImageDistEntry` to `SimilarImageEntry`.
pub fn dist_entry_to_api_sim_entry(dist: &ImageDistEntry, with_image: bool)
    -> SimilarImageEntry {

    let image_data = match with_image {
        false => None,
        true => {
            image::open(dist.image_name.clone())
                .map_err(|e| e.into()) 
                .and_then(|image: DynamicImage| image_to_base64(&image))
                .ok()
        },
    };

    let image_full_name = dist.image_name.clone();

    let image_name = match image_full_name.file_name() {
        None => "".to_owned(),
        Some(f) => f.to_string_lossy().into_owned(),
    };

    SimilarImageEntry { 
        image_name, 
        distance: dist.distance as f32, 
        data: image_data }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64() {

        // 8x7
        let small_png_1: String = 
            "iVBORw0KGgoAAAANSUhEUgAAAAgAAAAHCAIAAAC6O5sJAAAAGUlEQVR4nGJh+jWFARtgwio60BKAAAAA//8VUgGhHLHyHAAAAABJRU5ErkJggg==".to_owned();

        let im1_ = base64_to_image(&small_png_1).unwrap();

        assert_eq!((8, 7), (im1_.width(), im1_.height()));

        assert_eq!(im1_, base64_to_image(image_to_base64(&im1_).unwrap().as_str()).unwrap());

        ()
    }
}