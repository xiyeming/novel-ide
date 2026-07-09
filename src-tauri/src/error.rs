// src-tauri/src/error.rs
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("数据库错误: {0}")]
    Database(#[from] sqlx::Error),

    #[error("序列化错误: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("项目不存在: {0}")]
    ProjectNotFound(String),

    #[error("模型提供者不存在: {0}")]
    ProviderNotFound(String),

    #[error("连接测试失败: {0}")]
    ConnectionTestFailed(String),

    #[error("参数错误: {0}")]
    InvalidArgument(String),

    #[error("内部错误: {0}")]
    Internal(String),

    #[error("HTTP 请求错误: {0}")]
    Http(#[from] reqwest::Error),
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;
