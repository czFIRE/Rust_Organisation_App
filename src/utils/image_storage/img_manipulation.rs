use std::{io::Error, path::PathBuf};

use actix_multipart::form::tempfile::TempFile;
use uuid::Uuid;

use super::models::ImageCategory;

fn get_dir_by_category(category: ImageCategory) -> String {
    match category {
        ImageCategory::Company => "company".to_string(),
        ImageCategory::Event => "event".to_string(),
        ImageCategory::User => "user".to_string(),
    }
}

fn build_string_path(item_id: Uuid, category: String) -> String {
    let mut directory = "/img/".to_string();
    directory.push_str(category.as_str());
    directory.push('/');
    let mut user_file = item_id.to_string();
    user_file.push_str(".jpg");

    directory.push_str(user_file.as_str());

    directory
}

pub fn store_image(
    item_id: Uuid,
    category: ImageCategory,
    image: TempFile,
) -> Result<String, Error> {
    let mut path: PathBuf = PathBuf::new();
    path.push(".");
    path.push("src");
    path.push("static");
    path.push("img");
    let user_file = item_id.to_string();
    let directory = get_dir_by_category(category);
    path.push(directory.clone());
    path.push(user_file.clone());
    path.set_extension("jpg");

    let final_path = path.as_path();
    image.file.persist(final_path)?;

    Ok(build_string_path(item_id, directory))
}

pub fn remove_image(item_id: Uuid, category: ImageCategory) -> Result<(), Error> {
    let mut path: PathBuf = PathBuf::new();
    path.push(".");
    path.push("src");
    path.push("static");
    path.push("img");
    let user_file = item_id.to_string();
    let directory = get_dir_by_category(category);
    path.push(directory.clone());
    path.push(user_file.clone());
    path.set_extension("jpg");
    let final_path = path.as_path();
    std::fs::remove_file(final_path)
}
