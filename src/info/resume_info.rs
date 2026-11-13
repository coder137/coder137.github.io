use dioxus::prelude::manganis;
use dioxus::prelude::*;

use std::sync::LazyLock;

use crate::schema;

pub const ABOUT_IMAGE: Asset = asset!("/assets/about/about.jpg");

pub static RESUME_INFO: LazyLock<schema::ResumeInfo> = LazyLock::new(|| {
    let mut data = include_str!("resume_info.jsonc").to_string();
    json_strip_comments::strip(&mut data).expect(
        "Could not strip comments from resume_info.jsonc. Check if it is a valid jsonc file.",
    );
    serde_json::from_str(&data).expect("resume_info.jsonc is not a valid ResumeInfo schema.")
});
