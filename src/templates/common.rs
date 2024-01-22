use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub landing_title: String,
}

#[derive(Template)]
#[template(path = "register.html")]
pub struct RegistrationTemplate {
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
}

#[derive(Template)]
#[template(path = "error/error.html")]
pub struct ErrorTemplate {
    pub message: String,
}
