use aws_sdk_s3::{types::Object, Client};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct ObjectRange {
    pub part_num: i32,
    pub begin: usize,
    pub end: usize,
}

//Todo 尝试修改为Arc::<Client>
#[derive(Debug, Clone)]
pub struct OssClient {
    pub client: Client,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OssObjList {
    pub object_list: Option<Vec<Object>>,
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum OssProvider {
    JD,
    JRSS,
    ALI,
    S3,
    HUAWEI,
    COS,
    MINIO,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum S3RequestStyle {
    PathStyle,
    VirtualHostedStyle,
}

impl Default for S3RequestStyle {
    fn default() -> Self {
        S3RequestStyle::VirtualHostedStyle
    }
}

/// OSS 对象字符串列表，包含对象键名列表和下一页令牌
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct OssObjectsList {
    /// 对象键名列表
    pub object_list: Option<Vec<String>>,
    /// 下一页的令牌，用于分页查询
    pub next_token: Option<String>,
}

/// OSS 存储描述信息，用于配置和连接 OSS 服务
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OSSDescription {
    /// OSS 提供商类型（如 JD、ALI、S3 等）
    pub provider: OssProvider,
    /// 访问密钥 ID
    pub access_key_id: String,
    /// 秘密访问密钥
    pub secret_access_key: String,
    /// OSS 服务端点地址
    pub endpoint: String,
    /// 区域名称
    pub region: String,
    /// 存储桶名称
    pub bucket: String,
    /// 对象前缀（可选），用于过滤对象
    #[serde(default = "OSSDescription::prefix_default")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// 请求样式（路径样式或虚拟主机样式）
    #[serde(default = "OSSDescription::s3requeststyle")]
    pub request_style: S3RequestStyle,
}

impl Default for OSSDescription {
    fn default() -> Self {
        Self {
            provider: OssProvider::JD,
            access_key_id: "access_key_id".to_string(),
            secret_access_key: "secret_access_key".to_string(),
            endpoint: "http://s3.cn-north-1.jdcloud-oss.com".to_string(),
            region: "cn-north-1".to_string(),
            bucket: "bucket_name".to_string(),
            prefix: Some("test/samples/".to_string()),
            request_style: S3RequestStyle::default(),
        }
    }
}

impl OSSDescription {
    fn prefix_default() -> Option<String> {
        None
    }

    fn s3requeststyle() -> S3RequestStyle {
        S3RequestStyle::VirtualHostedStyle
    }
}
