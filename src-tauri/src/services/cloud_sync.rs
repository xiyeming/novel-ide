use crate::error::AppError;
use aliyun_oss::client::OSSClient;
use aliyun_oss::types::region::Region;
use reqwest::Client as ReqwestClient;
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
    client: ReqwestClient,
}

impl CloudSyncService {
    pub fn new() -> Self {
        Self {
            client: ReqwestClient::new(),
        }
    }

    pub async fn upload_to_oss(
        &self,
        endpoint: &str,
        bucket: &str,
        access_key: &str,
        secret_key: &str,
        local_path: &str,
        remote_path: &str,
    ) -> Result<(), AppError> {
        let region = Region::custom(endpoint, "custom");
        let client = OSSClient::builder()
            .region(region)
            .credentials(access_key, secret_key)
            .build()
            .map_err(|e| AppError::Internal(format!("OSS 客户端创建失败: {}", e)))?;
        let bucket_ops = client.bucket(bucket)
            .map_err(|e| AppError::Internal(format!("OSS Bucket 操作失败: {}", e)))?;
        let data = std::fs::read(local_path)?;
        bucket_ops.put_object(remote_path)
            .map_err(|e| AppError::Internal(format!("OSS PutObject 创建失败: {}", e)))?
            .body(data)
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("OSS 上传失败: {}", e)))?;
        Ok(())
    }

    pub async fn download_from_oss(
        &self,
        endpoint: &str,
        bucket: &str,
        access_key: &str,
        secret_key: &str,
        remote_path: &str,
        local_path: &str,
    ) -> Result<(), AppError> {
        let region = Region::custom(endpoint, "custom");
        let client = OSSClient::builder()
            .region(region)
            .credentials(access_key, secret_key)
            .build()
            .map_err(|e| AppError::Internal(format!("OSS 客户端创建失败: {}", e)))?;
        let bucket_ops = client.bucket(bucket)
            .map_err(|e| AppError::Internal(format!("OSS Bucket 操作失败: {}", e)))?;
        let output = bucket_ops.get_object(remote_path)
            .map_err(|e| AppError::Internal(format!("OSS GetObject 创建失败: {}", e)))?
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("OSS 下载失败: {}", e)))?;
        std::fs::write(local_path, output.body)?;
        Ok(())
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
            CloudProviderConfig::OSS { endpoint, bucket, access_key, secret_key } => {
                self.upload_to_oss(endpoint, bucket, access_key, secret_key, local_path, remote_path).await
            }
            CloudProviderConfig::S3 { .. } => {
                Err(AppError::Internal("S3 暂未实现".into()))
            }
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
            CloudProviderConfig::OSS { endpoint, bucket, access_key, secret_key } => {
                self.download_from_oss(endpoint, bucket, access_key, secret_key, remote_path, local_path).await
            }
            CloudProviderConfig::S3 { .. } => {
                Err(AppError::Internal("S3 暂未实现".into()))
            }
        }
    }
}
