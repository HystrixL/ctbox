use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename(deserialize = "USERFLOW"))]
    user_flow: f64,
    #[serde(rename(deserialize = "USERTIME"))]
    user_time: f64,
    #[serde(rename(deserialize = "USERMONEY"))]
    user_money: f64,
    #[serde(rename(deserialize = "MAC"))]
    mac: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    login_time: String,
    bas_id: i32,
    login_ip: String,
    mac_address: String,
}