use std::{io::Error, path::Path};

use actix_multipart::form::tempfile::TempFile;
use uuid::Uuid;

use super::models::ImageCategory;

fn get_dir_by_category(category: ImageCategory) -> String {
    match category {
        ImageCategory::Company => "/img/company/".to_string(),
        ImageCategory::Event => "/img/event/".to_string(),
        ImageCategory::User => "/img/user/".to_string(),
    }
}

pub fn store_image(
    item_id: Uuid,
    category: ImageCategory,
    image: TempFile,
) -> Result<String, Error> {
    let mut user_file = item_id.to_string();
    user_file.push_str(".jpg");

    let mut directory = get_dir_by_category(category);
    directory.push_str(user_file.as_str());

    let mut path_string = "./src/static".to_string();
    path_string.push_str(directory.as_str());

    let path = Path::new(path_string.as_str());
    image.file.persist(path)?;
    Ok(directory)
}

pub fn remove_image(item_id: Uuid, category: ImageCategory) -> Result<(), Error> {
    let mut user_file = item_id.to_string();
    user_file.push_str(".jpg");

    let mut directory = get_dir_by_category(category);
    directory.push_str(user_file.as_str());

    let mut path_string = "./src/static".to_string();
    path_string.push_str(directory.as_str());

    let path = Path::new(path_string.as_str());
    std::fs::remove_file(path)
}
