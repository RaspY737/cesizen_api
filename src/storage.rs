use aws_config::BehaviorVersion;
use aws_sdk_s3::Client;
use aws_sdk_s3::config::{Credentials, Region};
use aws_sdk_s3::primitives::ByteStream;
use aws_smithy_types::body::SdkBody;

use crate::config::S3Config;
use crate::errors::AppError;

#[derive(Clone)]
pub struct S3Storage {
    client: Client,
    bucket: String,
    endpoint: String,
}

impl S3Storage {
    pub async fn new(config: &S3Config) -> Self {
        let credentials =
            Credentials::new(&config.access_key, &config.secret_key, None, None, "env");

        let s3_config = aws_sdk_s3::Config::builder()
            .behavior_version(BehaviorVersion::latest())
            .region(Region::new(config.region.clone()))
            .endpoint_url(&config.endpoint)
            .credentials_provider(credentials)
            .force_path_style(true)
            .build();

        let client = Client::from_conf(s3_config);

        Self {
            client,
            bucket: config.bucket.clone(),
            endpoint: config.endpoint.clone(),
        }
    }

    pub async fn upload(
        &self,
        key: &str,
        data: Vec<u8>,
        content_type: &str,
    ) -> Result<String, AppError> {
        let body = ByteStream::new(SdkBody::from(data));

        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(body)
            .content_type(content_type)
            .send()
            .await
            .map_err(|e| {
                log::error!("S3 upload error: {e:?}");
                AppError::Internal("Erreur lors de l'upload du fichier".to_string())
            })?;

        Ok(self.public_url(key))
    }

    pub async fn delete(&self, key: &str) -> Result<(), AppError> {
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| {
                log::error!("S3 delete error: {e:?}");
                AppError::Internal("Erreur lors de la suppression du fichier".to_string())
            })?;

        Ok(())
    }

    pub async fn download(&self, key: &str) -> Result<String, AppError> {
        let resp = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| {
                log::error!("S3 download error: {e:?}");
                AppError::Internal("Erreur lors du téléchargement du fichier".to_string())
            })?;

        let bytes = resp.body.collect().await.map_err(|e| {
            log::error!("S3 read body error: {e:?}");
            AppError::Internal("Erreur lors de la lecture du fichier".to_string())
        })?;

        String::from_utf8(bytes.to_vec())
            .map_err(|_| AppError::Internal("Le fichier n'est pas du texte valide".to_string()))
    }

    pub fn public_url(&self, key: &str) -> String {
        format!("{}/{}/{}", self.endpoint, self.bucket, key)
    }
}
