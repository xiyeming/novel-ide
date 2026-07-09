# 设计文档：设置页面、云存储、死代码清理

## 1. 概述

本设计文档涵盖三个待完成的功能：
1. 设置 Tab 全部设置
2. OSS/S3 云存储实现
3. 死代码清理

## 2. 设置 Tab 全部设置

### 2.1 目标

创建一个完整的设置页面，包含所有可配置项。

### 2.2 包含模块

#### 编辑器设置
- 字体选择（默认：JetBrains Mono）
- 字号（默认：14px）
- 主题（暗色/亮色/自定义）
- 自动保存间隔（默认：30秒）
- 行高（默认：1.6）

#### AI 设置
- 默认模型选择
- 温度（0-2，默认：0.7）
- 最大 token 数（默认：2000）
- 系统提示词

#### 导出设置
- 默认导出格式（TXT/MD/DOCX/PDF/EPUB）
- 导出路径
- 包含元数据

### 2.3 数据结构

```typescript
interface AppSettings {
  // 编辑器
  editor_font: string;
  editor_font_size: number;
  editor_line_height: number;
  auto_save_interval: number; // 秒
  
  // AI
  default_model_id: string;
  ai_temperature: number;
  ai_max_tokens: number;
  ai_system_prompt: string;
  
  // 导出
  default_export_format: string;
  export_path: string;
  export_include_metadata: boolean;
}
```

### 2.4 UI 设计

使用卡片式布局，每个模块一个卡片：

```
┌─────────────────────────────────────────┐
│  编辑器设置                              │
│  ┌─────────────────────────────────────┐ │
│  │ 字体: [JetBrains Mono      ▼]      │ │
│  │ 字号: [14          ] px            │ │
│  │ 行高: [1.6         ]               │ │
│  │ 自动保存: [30         ] 秒         │ │
│  └─────────────────────────────────────┘ │
├─────────────────────────────────────────┤
│  AI 设置                                 │
│  ┌─────────────────────────────────────┐ │
│  │ 默认模型: [DeepSeek Chat   ▼]      │ │
│  │ 温度: [0.7         ] (0-2)         │ │
│  │ 最大 Token: [2000        ]         │ │
│  │ 系统提示词:                        │ │
│  │ [你是一个专业的中文小说写作助手...]  │ │
│  └─────────────────────────────────────┘ │
├─────────────────────────────────────────┤
│  导出设置                                │
│  ┌─────────────────────────────────────┐ │
│  │ 默认格式: [TXT           ▼]        │ │
│  │ 导出路径: [/home/user/exports]      │ │
│  │ 包含元数据: [✓]                     │ │
│  └─────────────────────────────────────┘ │
└─────────────────────────────────────────┘
```

## 3. OSS/S3 云存储

### 3.1 目标

实现阿里云 OSS 和 AWS S3 的文件上传/下载功能。

### 3.2 依赖

```toml
[dependencies]
rusoto_s3 = "0.48.0"
rusoto_core = "0.48.0"
aliyun-oss = "0.10.0"
```

### 3.3 实现

#### 阿里云 OSS

```rust
use aliyun_oss::Client;

pub async fn upload_to_oss(
    endpoint: &str,
    bucket: &str,
    access_key: &str,
    secret_key: &str,
    local_path: &str,
    remote_path: &str,
) -> Result<(), AppError> {
    let client = Client::new(endpoint, bucket, access_key, secret_key)?;
    let data = std::fs::read(local_path)?;
    client.put_object(remote_path, data).await?;
    Ok(())
}
```

#### AWS S3

```rust
use rusoto_s3::{S3, S3Client, PutObjectRequest, GetObjectRequest};
use rusoto_core::{Region, credential::StaticProvider};

pub async fn upload_to_s3(
    endpoint: &str,
    bucket: &str,
    access_key: &str,
    secret_key: &str,
    region: &str,
    local_path: &str,
    remote_path: &str,
) -> Result<(), AppError> {
    let credentials = StaticProvider::new(access_key.into(), secret_key.into(), None, None);
    let region = Region::Custom {
        endpoint: endpoint.into(),
        name: region.into(),
    };
    let client = S3Client::new_with_client(
        rusoto_core::request::HttpClient::new()?,
        credentials,
        region,
    );
    
    let data = std::fs::read(local_path)?;
    let req = PutObjectRequest {
        bucket: bucket.into(),
        key: remote_path.into(),
        body: Some(data.into()),
        ..Default::default()
    };
    
    client.put_object(req).await?;
    Ok(())
}
```

### 3.4 CloudSyncService 更新

```rust
pub async fn upload_file(
    &self,
    config: &CloudConfig,
    local_path: &str,
    remote_path: &str,
) -> Result<(), AppError> {
    match &config.config {
        CloudProviderConfig::WebDAV { url, username, password } => {
            // 已实现
        }
        CloudProviderConfig::OSS { endpoint, bucket, access_key, secret_key } => {
            upload_to_oss(endpoint, bucket, access_key, secret_key, local_path, remote_path).await
        }
        CloudProviderConfig::S3 { endpoint, bucket, access_key, secret_key, region } => {
            upload_to_s3(endpoint, bucket, access_key, secret_key, region, local_path, remote_path).await
        }
    }
}
```

## 4. 死代码清理

### 4.1 HyprlandService

**分析：**
- `services/hyprland.rs` 中的功能已在 `commands/shortcuts.rs` 中实现
- `generate_hyprland_config` 命令已经替代了 `HyprlandService` 的功能

**决策：** 删除 `services/hyprland.rs`

### 4.2 optimize_content

**分析：**
- `PerformanceService::optimize_content` 没有在任何地方被使用
- 其他方法（`count_lines`、`read_file_chunk`、`get_file_size`）已被使用

**决策：** 删除 `optimize_content` 方法

## 5. 实现步骤

1. **Phase 11: 设置页面** (3 tasks)
   - 创建 SettingsPanel.vue 组件
   - 实现编辑器/AI/导出设置
   - 添加保存/加载逻辑

2. **Phase 12: 云存储** (3 tasks)
   - 添加 Rust 依赖
   - 实现 OSS/S3 上传/下载
   - 测试云存储功能

3. **Phase 13: 死代码清理** (2 tasks)
   - 删除 HyprlandService
   - 删除 optimize_content

## 6. 验证标准

- [ ] 设置页面显示所有配置项
- [ ] 设置可保存和加载
- [ ] OSS 上传/下载功能正常
- [ ] S3 上传/下载功能正常
- [ ] 死代码已删除
- [ ] 无编译警告
- [ ] 所有测试通过
