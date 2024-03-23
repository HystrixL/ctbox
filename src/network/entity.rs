use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename(deserialize = "USERFLOW"))]
    pub user_flow: f64,
    #[serde(rename(deserialize = "USERTIME"))]
    pub user_time: f64,
    #[serde(rename(deserialize = "USERMONEY"))]
    pub user_money: f64,
    #[serde(rename(deserialize = "MAC"))]
    pub mac: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    pub login_time: String,
    pub bas_id: i32,
    pub login_ip: String,
    pub mac_address: String,
}