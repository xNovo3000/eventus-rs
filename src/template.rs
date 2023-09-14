use askama::Template;
use chrono::{DateTime, Local};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub time: DateTime<Local>
}