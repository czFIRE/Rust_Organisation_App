use askama::Template;

#[derive(Template)]
#[template(path = "avatar/avatar.html")]
pub struct AvatarTemplate {
    pub avatar_url: String,
}
