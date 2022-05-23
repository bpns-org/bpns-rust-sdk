// Copyright (c) 2021-2022 Yuki Kishimoto
// Distributed under the MIT software license

#[derive(Deserialize, Debug)]
pub struct GenericResult<T> {
    pub success: bool,
    pub code: u16,
    pub message: String,
    pub data: T,
}

#[derive(Deserialize, Debug)]
pub struct EmptyData {}

#[derive(Deserialize, Debug)]
pub struct GenerateNewToken {
    pub token: String,
}

#[derive(Deserialize, Debug)]
pub struct Ping {
    pub name: String,
    pub version: String,
}

#[derive(Deserialize, Debug)]
pub struct Notification {
    pub id: String,
    pub address: String,
    pub txid: String,
    pub txtype: String,
    pub amount: u64,
    pub confirmed: bool,
    pub timestamp: u64,
}

#[derive(Deserialize, Debug)]
pub struct Notifications {
    pub notifications: Vec<Notification>,
}

#[derive(Deserialize, Debug)]
pub struct Addresses {
    pub addresses: Vec<String>,
}
