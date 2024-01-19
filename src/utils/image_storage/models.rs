use actix_multipart::form::{tempfile::TempFile, MultipartForm};

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(rename = "file")]
    pub file: TempFile,
}

pub enum ImageCategory {
    User,
    Company,
    Event,
}

pub const MAX_FILE_SIZE: usize = 1024 * 1024 * 10;
pub const DEFAULT_USER_IMAGE: &str = "/img/default/user.jpg";
pub const DEFAULT_COMPANY_IMAGE: &str = "/img/default/company.jpg";
pub const DEFAULT_EVENT_IMAGE: &str = "/img/default/event.jpg";
