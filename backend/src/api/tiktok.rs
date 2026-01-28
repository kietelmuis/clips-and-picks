use std::error::Error;

use bytes::Bytes;
use reqwest::{Client, StatusCode, header::CONTENT_TYPE};
use serde_derive::Deserialize;
use serde_json::json;
use uuid::Uuid;

#[derive(Clone)]
pub struct TiktokApi {
    client: Client,
    auth: String,
}

#[derive(Deserialize)]
pub struct RequestData {
    pub request_id: u32,
}

#[derive(Deserialize)]
pub struct RequestResponse {
    pub data: RequestData,
}

#[derive(Deserialize)]
pub struct RequestStatusData {
    status: String,
}

#[derive(Deserialize)]
pub struct RequestStatus {
    pub data: RequestStatusData,
}

#[derive(Deserialize)]
pub struct RequestDownload {
    pub data: RequestStatusData,
}

const BASEURL: &str = "https://open.tiktokapis.com/v2";

impl TiktokApi {
    pub fn new(auth: String) -> Self {
        Self {
            client: Client::builder().build().unwrap(),
            auth,
        }
    }

    pub async fn request_data(&self) -> Result<RequestResponse, Box<dyn Error>> {
        let response = self
            .client
            .post(format!("{}/user/data/add/", BASEURL))
            .query(&[("fields", Uuid::new_v4().to_string())])
            .body(
                json!({
                    "data_format": "json",
                    "category_selection_list": vec!["activity"]
                })
                .to_string(),
            )
            .bearer_auth(&self.auth)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => Ok(response.json::<RequestResponse>().await?),
            s => Err(format!("[api] failed to request data: {}", s).into()),
        }
    }

    pub async fn request_data_status(
        &self,
        request_id: String,
    ) -> Result<RequestStatus, Box<dyn Error>> {
        let response = self
            .client
            .post(format!("{}/user/data/check/", BASEURL))
            .query(&[("fields", "status")])
            .body(
                json!({
                    "request_id": request_id
                })
                .to_string(),
            )
            .bearer_auth(&self.auth)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => Ok(response.json::<RequestStatus>().await?),
            s => Err(format!("[api] failed to get status of data request: {}", s).into()),
        }
    }

    pub async fn download_request_data(&self, request_id: String) -> Result<Bytes, Box<dyn Error>> {
        let response = self
            .client
            .post(format!("{}/user/data/download/", BASEURL))
            .body(
                json!({
                    "request_id": request_id
                })
                .to_string(),
            )
            .bearer_auth(&self.auth)
            .header(CONTENT_TYPE, "application/json")
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => Ok(response.bytes().await?),
            s => Err(format!("[api] failed to download request data: {}", s).into()),
        }
    }
}
