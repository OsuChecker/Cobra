use std::error::Error;
use futures_util::StreamExt;
use reqwest::{Client, Response, Method, StatusCode};
use tokio::io::AsyncWriteExt;
use serde::de::DeserializeOwned;
use crate::MapSetResponse;

/// Custom result type for API operations that can return any error type
type ApiResult<T> = Result<T, Box<dyn Error>>;
/// Custom result type specifically for operations returning strings
type ApiStringResult = Result<String, String>;

/// Represents an API client for making HTTP requests
#[derive(Clone)]
pub struct Api {
    /// HTTP client instance
    client: Client,
    /// Base URL for all API requests
    base_url: String,
}

impl Api {
    /// Creates a new API client instance with default configuration
    pub fn new() -> Self {
        let client: Client = Client::builder()
            .user_agent("Cobra/0.2.0")
            .build()
            .expect("Impossible de construire le client Reqwest");
        Self {
            client,
            base_url: "https://osef.me/api".to_string()
        }
    }

    /// Makes a generic HTTP request to the API
    ///
    /// # Arguments
    /// * `method` - HTTP method to use
    /// * `endpoint` - API endpoint to call
    /// * `body` - Optional JSON body to send with the request
    async fn request<T>(
        &self,
        method: Method,
        endpoint: &str,
        body: Option<serde_json::Value>
    ) -> ApiResult<Response>
    where
        T: DeserializeOwned + Send + 'static
    {
        let url: String = format!("{}{}", self.base_url, endpoint);
        let mut builder = self.client.request(method, &url);

        if let Some(json) = body {
            builder = builder.json(&json);
        }

        let response: Response = builder.send().await?;
        Ok(response)
    }

    /// Fetches an image from a given URL
    ///
    /// # Arguments
    /// * `url` - URL of the image to fetch
    pub async fn fetch_image(&self, url: &str) -> ApiResult<Vec<u8>> {
        let response: Response = self.client.get(url).send().await?;
        let bytes: bytes::Bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }

    /// Fetches a page of mapsets from the API
    ///
    /// # Arguments
    /// * `page` - Page number to fetch
    pub async fn fetch_mapsets(&self, page: i32) -> ApiResult<MapSetResponse> {
        let response: Response = self.request::<MapSetResponse>(
            Method::GET,
            &format!("/mapset?page={}", page),
            None
        ).await?;

        let mapset: MapSetResponse = response.json().await?;
        Ok(mapset)
    }

    /// Verifies user credentials against the API
    ///
    /// # Arguments
    /// * `username` - User's username/UUID
    /// * `password` - User's password
    pub async fn check_credentials(&self, username: &str, password: &str) -> ApiStringResult {
        let params: serde_json::Value = serde_json::json!({
            "uuid": username,
            "password": password
        });

        let response: Response = self.request::<String>(Method::POST, "/login", Some(params))
            .await
            .map_err(|e| format!("Erreur HTTP: {}", e))?;

        if response.status().is_success() {
            let token: String = response.text()
                .await
                .map_err(|e| format!("Erreur de lecture de la réponse: {}", e))?;
            Ok(token)
        } else {
            let status: StatusCode = response.status();
            Err(format!("Identifiants incorrects (status: {})", status))
        }
    }

    /// Downloads a file from a URL to a local path with progress tracking
    ///
    /// # Arguments
    /// * `url` - URL of the file to download
    /// * `path` - Local path where the file should be saved
    /// * `progress_callback` - Callback function that receives download progress updates
    pub async fn download_files(
        &self,
        url: &str,
        path: &str,
        progress_callback: impl Fn(f32) + Send + 'static,
    ) -> ApiResult<()> {
        let response: Response = self.client.get(url).send().await?;
        let total_size: u64 = response.content_length().unwrap_or(0);

        let mut file = tokio::fs::File::create(path).await?;
        let mut downloaded: u64 = 0;
        let mut stream = response.bytes_stream();

        while let Some(chunk_result) = stream.next().await {
            let chunk: bytes::Bytes = chunk_result?;
            downloaded += chunk.len() as u64;

            if total_size > 0 {
                let progress: f32 = downloaded as f32 / total_size as f32;
                progress_callback(progress);
            }

            file.write_all(&chunk).await?;
        }

        Ok(())
    }
}