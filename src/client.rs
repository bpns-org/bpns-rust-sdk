// Copyright (c) 2021-2022 Yuki Kishimoto
// Distributed under the MIT software license

use std::collections::HashMap;

use reqwest::header::{HeaderMap, HeaderValue};
use serde::de::DeserializeOwned;
use serde_json::json;

use crate::model::{EmptyData, GenerateNewToken, GenericResult, Addresses, Notifications, Notification, Ping};

#[derive(Clone)]
pub struct Client {
    endpoint: String,
    token: String,
    client: reqwest::Client,
}

#[derive(Debug)]
pub enum ClientError {
    FailedToDeserialize(String),
    ReqwestError(reqwest::Error),
    BadRequest(String),
    EmptyResponse,
    BadResult,
}

impl Client {
    pub fn new(host: &str, token: &str) -> Self {
        let mut headers = HeaderMap::new();
        let mut auth_value = HeaderValue::from_str(token).unwrap();
        auth_value.set_sensitive(true);
        headers.insert("BPNS-Auth-Token", auth_value);

        let client = reqwest::Client::builder().default_headers(headers);

        Self {
            endpoint: host.into(),
            token: token.into(),
            client: client.build().unwrap(),
        }
    }

    pub async fn generate_new_token(host: &str) -> Result<String, ClientError> {
        let client = reqwest::Client::new();

        let endpoint: String = format!("{}/newPushNotificationToken", host);

        let req = client.get(endpoint);
        let res = request::<GenerateNewToken>(req).await?;

        Ok(res.data.token)
    }

    pub async fn ping(&self) -> Result<Ping, ClientError> {
        let endpoint: String = format!("{}/ping", self.endpoint);

        let req = self.client.get(endpoint);
        let res = request::<Ping>(req).await?;

        Ok(res.data)
    }

    pub async fn subscribe(&self) -> Result<(), ClientError> {
        let endpoint: String = format!("{}/subscribe/{}", self.endpoint, self.token);

        let req = self.client.post(endpoint);
        request::<EmptyData>(req).await?;

        Ok(())
    }

    pub async fn unsubscribe(&self) -> Result<(), ClientError> {
        let endpoint: String = format!("{}/unsubscribe", self.endpoint);

        let req = self.client.post(endpoint);
        request::<EmptyData>(req).await?;

        Ok(())
    }

    pub async fn notifications(&self) -> Result<Vec<Notification>, ClientError> {
        let endpoint: String = format!("{}/notifications", self.endpoint);

        let req = self.client.get(endpoint);
        let res = request::<Notifications>(req).await?;

        Ok(res.data.notifications)
    }

    pub async fn delete_notifications(&self) -> Result<(), ClientError> {
        let endpoint: String = format!("{}/notifications", self.endpoint);

        let req = self.client.delete(endpoint);
        request::<EmptyData>(req).await?;

        Ok(())
    }

    pub async fn delete_notification_by_id(&self, id: &str) -> Result<(), ClientError> {
        let endpoint: String = format!("{}/notification/{}", self.endpoint, id);

        let req = self.client.delete(endpoint);
        request::<EmptyData>(req).await?;

        Ok(())
    }

    pub async fn addresses(&self) -> Result<Vec<String>, ClientError> {
        let endpoint: String = format!("{}/addresses", self.endpoint);

        let req = self.client.get(endpoint);
        let res = request::<Addresses>(req).await?;

        Ok(res.data.addresses)
    }

    pub async fn add_addresses(&self, addresses: &[&str]) -> Result<(), ClientError> {
        let endpoint: String = format!("{}/addresses", self.endpoint);

        let mut map = HashMap::new();
        map.insert("addresses", addresses);

        let req = self.client.post(endpoint).json(&map);
        request::<EmptyData>(req).await?;

        Ok(())
    }

    pub async fn delete_addresses(&self, addresses: &[&str]) -> Result<(), ClientError> {
        let endpoint: String = format!("{}/addresses", self.endpoint);

        let mut map = HashMap::new();
        map.insert("addresses", addresses);

        let req = self.client.delete(endpoint).json(&map);
        request::<EmptyData>(req).await?;

        Ok(())
    }

    pub async fn add_addresses_from_singlesig(
        &self,
        public_key: &str,
        from_index: u32,
        to_index: u32,
        is_change: bool,
    ) -> Result<(), ClientError> {
        let endpoint: String = format!("{}/addresses/singlesig", self.endpoint);

        let body = json!({
            "public_key": public_key,
            "from_index": from_index,
            "to_index": to_index,
            "is_change": is_change
        });

        let req = self.client.post(endpoint).json(&body);
        request::<EmptyData>(req).await?;

        Ok(())
    }

    pub async fn delete_addresses_from_singlesig(
        &self,
        public_key: &str,
        from_index: u32,
        to_index: u32,
        is_change: bool,
    ) -> Result<(), ClientError> {
        let endpoint: String = format!("{}/addresses/singlesig", self.endpoint);

        let body = json!({
            "public_key": public_key,
            "from_index": from_index,
            "to_index": to_index,
            "is_change": is_change
        });

        let req = self.client.delete(endpoint).json(&body);
        request::<EmptyData>(req).await?;

        Ok(())
    }

    pub async fn add_addresses_from_multisig(
        &self,
        script_type: &str,
        required_signatures: u8,
        public_keys: &[&str],
        from_index: u32,
        to_index: u32,
        is_change: bool,
    ) -> Result<(), ClientError> {
        let endpoint: String = format!("{}/addresses/multisig", self.endpoint);

        let body = json!({
            "script_type": script_type,
            "required_signatures": required_signatures,
            "public_keys": public_keys,
            "from_index": from_index,
            "to_index": to_index,
            "is_change": is_change
        });

        let req = self.client.post(endpoint).json(&body);
        request::<EmptyData>(req).await?;

        Ok(())
    }

    pub async fn delete_addresses_from_multisig(
        &self,
        script_type: &str,
        required_signatures: u8,
        public_keys: &[&str],
        from_index: u32,
        to_index: u32,
        is_change: bool,
    ) -> Result<(), ClientError> {
        let endpoint: String = format!("{}/addresses/multisig", self.endpoint);

        let body = json!({
            "script_type": script_type,
            "required_signatures": required_signatures,
            "public_keys": public_keys,
            "from_index": from_index,
            "to_index": to_index,
            "is_change": is_change
        });

        let req = self.client.delete(endpoint).json(&body);
        request::<EmptyData>(req).await?;

        Ok(())
    }
}

async fn request<T>(req: reqwest::RequestBuilder) -> Result<GenericResult<T>, ClientError>
where
    T: DeserializeOwned,
{
    let data = req.send().await?;
    let res = data.text().await?;

    if res.is_empty() {
        return Err(ClientError::EmptyResponse);
    }

    deserialize::<T>(res.as_str())
}

fn deserialize<T>(data: &str) -> Result<GenericResult<T>, ClientError>
where
    T: DeserializeOwned,
{
    match serde_json::from_str::<GenericResult<T>>(data) {
        Ok(res) => {
            if !res.success {
                return Err(ClientError::BadRequest(res.message));
            }

            Ok(res)
        }
        Err(error) => Err(ClientError::FailedToDeserialize(error.to_string())),
    }
}

impl From<reqwest::Error> for ClientError {
    fn from(err: reqwest::Error) -> Self {
        ClientError::ReqwestError(err)
    }
}
