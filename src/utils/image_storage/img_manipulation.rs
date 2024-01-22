use std::{
    io::{
        self, Error,
    },
    path::PathBuf,
    fs,
};

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

    // Get a `std::fs::File` representation.
    let mut src_file = image.file.into_file();

    //
    // Write the temporary file content into a destination file.
    //
    // Note: This truncates the destination file if it already exists.
    //
    // Note: Beware that methods must be called in specific order:
    //       https://github.com/rust-lang/rust/issues/90634
    //
    // Note: Do **not** use TempFile.file.persist() for this
    //       as this method fails with OS error 18 (Invalid cross-device link)
    //       when source and destination are not at same partition.
    //
    let mut dest_file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .read(true)
        .open(final_path.clone())?;

    io::copy(&mut src_file, &mut dest_file)?;

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
