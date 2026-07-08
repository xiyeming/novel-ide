// src-tauri/src/db/models/mod.rs
pub mod project;
pub mod settings;

pub use project::{CreateProjectRequest, Project};
pub use settings::{GlobalSetting, ProjectSetting};
