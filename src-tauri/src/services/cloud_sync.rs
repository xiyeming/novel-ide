use crate::error::AppError;
use aliyun_oss::client::OSSClient;
use aliyun_oss::types::region::Region;
use reqwest::Client as ReqwestClient;
use rusoto_core::credential::StaticProvider;
use rusoto_core::{Client, Region as RusotoRegion};
use rusoto_s3::{GetObjectRequest, PutObjectRequest, S3, S3Client};
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

    pub async fn upload_to_s3(
        &self,
        endpoint: &str,
        bucket: &str,
        access_key: &str,
        secret_key: &str,
        region: &str,
        local_path: &str,
        remote_path: &str,
    ) -> Result<(), AppError> {
        let credentials = StaticProvider::new(
            access_key.into(),
            secret_key.into(),
            None,
            None,
        );
        let region = RusotoRegion::Custom {
            endpoint: endpoint.into(),
            name: region.into(),
        };
        let http_client = rusoto_core::request::HttpClient::new()
            .map_err(|e| AppError::Internal(format!("HTTP 客户端创建失败: {}", e)))?;
        let client = S3Client::new_with_client(Client::new_with(credentials, http_client), region);

        let data = std::fs::read(local_path)?;
        let req = PutObjectRequest {
            bucket: bucket.into(),
            key: remote_path.into(),
            body: Some(data.into()),
            ..Default::default()
        };

        client
            .put_object(req)
            .await
            .map_err(|e| AppError::Internal(format!("S3 上传失败: {}", e)))?;
        Ok(())
    }

    pub async fn download_from_s3(
        &self,
        endpoint: &str,
        bucket: &str,
        access_key: &str,
        secret_key: &str,
        region: &str,
        remote_path: &str,
        local_path: &str,
    ) -> Result<(), AppError> {
        let credentials = StaticProvider::new(
            access_key.into(),
            secret_key.into(),
            None,
            None,
        );
        let region = RusotoRegion::Custom {
            endpoint: endpoint.into(),
            name: region.into(),
        };
        let http_client = rusoto_core::request::HttpClient::new()
            .map_err(|e| AppError::Internal(format!("HTTP 客户端创建失败: {}", e)))?;
        let client = S3Client::new_with_client(Client::new_with(credentials, http_client), region);

        let req = GetObjectRequest {
            bucket: bucket.into(),
            key: remote_path.into(),
            ..Default::default()
        };

        let result = client
            .get_object(req)
            .await
            .map_err(|e| AppError::Internal(format!("S3 下载失败: {}", e)))?;

        use futures_util::TryStreamExt;
        let body = result
            .body
            .ok_or_else(|| AppError::Internal("S3 响应无内容".into()))?;
        let bytes = body
            .try_fold(Vec::new(), |mut acc, chunk| async move {
                acc.extend_from_slice(&chunk);
                Ok::<_, std::io::Error>(acc)
            })
            .await
            .map_err(|e| AppError::Internal(format!("S3 读取失败: {}", e)))?;

        std::fs::write(local_path, bytes)?;
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
            CloudProviderConfig::S3 { endpoint, bucket, access_key, secret_key, region } => {
                self.upload_to_s3(endpoint, bucket, access_key, secret_key, region, local_path, remote_path).await
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
            CloudProviderConfig::S3 { endpoint, bucket, access_key, secret_key, region } => {
                self.download_from_s3(endpoint, bucket, access_key, secret_key, region, remote_path, local_path).await
            }
        }
    }
}
