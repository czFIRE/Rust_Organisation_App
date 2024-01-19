use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub landing_title: String,
}

#[derive(Template)]
#[template(path = "error/error.html")]
pub struct ErrorTemplate {
    pub message: String,
}
