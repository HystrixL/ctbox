use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
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
pub struct DeviceData {
    pub login_time: String,
    pub bas_id: i32,
    pub login_ip: String,
    pub mac_address: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct User {
    pub account: String,
    pub password: String,
}

impl User{
    pub fn new(account:String,password:String)->Self{
        User { account, password }
    }
}
