use crate::error::AppError;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudConfig {
    pub id: String,
    pub name: String,
    pub provider_type: String,
    pub config: CloudProviderConfig,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CloudProviderConfig {
    WebDAV {
        url: String,
        username: String,
        password: String,
    },
    OSS {
        endpoint: String,
        bucket: String,
        access_key: String,
        secret_key: String,
    },
    S3 {
        endpoint: String,
        bucket: String,
        access_key: String,
        secret_key: String,
        region: String,
    },
}

pub struct CloudSyncService {
    client: Client,
}

impl CloudSyncService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn upload_file(
        &self,
        config: &CloudConfig,
        local_path: &str,
        remote_path: &str,
    ) -> Result<(), AppError> {
        match &config.config {
            CloudProviderConfig::WebDAV { url, username, password } => {
                let full_url = format!("{}/{}", url.trim_end_matches('/'), remote_path);
                let body = std::fs::read(local_path)?;
                let response = self.client
                    .put(&full_url)
                    .basic_auth(username, Some(password))
                    .body(body)
                    .send()
                    .await?;

                if !response.status().is_success() {
                    return Err(AppError::Internal(format!("上传失败: {}", response.status())));
                }
                Ok(())
            }
            _ => Err(AppError::Internal("暂不支持此云存储类型".into())),
        }
    }

    pub async fn download_file(
        &self,
        config: &CloudConfig,
        remote_path: &str,
        local_path: &str,
    ) -> Result<(), AppError> {
        match &config.config {
            CloudProviderConfig::WebDAV { url, username, password } => {
                let full_url = format!("{}/{}", url.trim_end_matches('/'), remote_path);
                let response = self.client
                    .get(&full_url)
                    .basic_auth(username, Some(password))
                    .send()
                    .await?;

                if !response.status().is_success() {
                    return Err(AppError::Internal(format!("下载失败: {}", response.status())));
                }

                let bytes = response.bytes().await?;
                std::fs::write(local_path, bytes)?;
                Ok(())
            }
            _ => Err(AppError::Internal("暂不支持此云存储类型".into())),
        }
    }
}
