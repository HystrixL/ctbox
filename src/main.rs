use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{error::Error, process::Command};

const ENTRANCE_IP: &'static str = "wifi.cnu.edu.cn";

const LOGIN_NODE: &'static str = "/drcom/login";
const LOGIN_CALLBACK: &'static str = "dr1004";
const LOGIN_0MKKEY: &'static str = "123456";

const LOGOUT_NODE: &'static str = "/drcom/logout";
const LOGOUT_CALLBACK: &'static str = "dr1004";

const QUERY_USER_INFO_NODE: &'static str = ":802/eportal/portal/custom/loadUserInfo";
const QUERY_USER_INFO_CALLBACK: &'static str = "dr1002";
const QUERY_ONLINE_DEVICE_NODE: &'static str = ":802/eportal/portal/custom/loadOnlineDevice";
const QUERY_ONLINE_DEVICE_CALLBACK: &'static str = "dr1003";

#[derive(Serialize, Deserialize, Debug)]
struct LoginResult {
    result: i32,
    v46ip: String,
    olmac: String,
    uid: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct LogoutResult {
    result: i32,
    ss5: String,
    ss6: String,
    ss4: String,
    time: i32,
    flow: f64,
    uid: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct UserInfo {
    #[serde(rename(deserialize = "USERFLOW"))]
    user_flow: f64,
    #[serde(rename(deserialize = "USERTIME"))]
    user_time: f64,
    #[serde(rename(deserialize = "USERMONEY"))]
    user_money: f64,
    #[serde(rename(deserialize = "MAC"))]
    mac: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct QueryUserInfoResult {
    code: String,
    data: Vec<UserInfo>,
    msg: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct DeviceInfo {
    login_time: String,
    bas_id: i32,
    login_ip: String,
    mac_address: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct QueryDeviceInfoResult {
    code: String,
    data: Vec<DeviceInfo>,
    msg: String,
}

#[derive(Parser)]
#[command(version, author, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    sub_action: Option<SubAction>,
}

#[derive(Subcommand)]
enum SubAction {
    /// 校园网相关操作
    Network {
        #[command(subcommand)]
        network_action: NetworkAction,
    },
}

#[derive(Subcommand)]
enum NetworkAction {
    /// 登录
    Login {
        /// 账号
        #[arg(short, long)]
        account: String,
        /// 密码
        #[arg(short, long)]
        password: String,
    },
    /// 登出
    Logout {},
    /// 查询
    Status {
        /// 账号
        #[arg(short, long)]
        account: Option<String>,
    },
}

fn is_cnu() -> bool {
    #[cfg(target_os = "linux")]
    let output = Command::new("nmcli")
        .arg("-g")
        .arg("NAME")
        .arg("connection")
        .arg("show")
        .arg("--active")
        .output()
        .unwrap();

    #[cfg(target_os = "windows")]
    let output = Command::new("netsh")
        .arg("WLAN")
        .arg("show")
        .arg("interfaces")
        .output()
        .unwrap();

    let out = String::from_utf8(output.stdout).unwrap();
    out.split("\n").any(|v| v == "CNU")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    if !is_cnu(){
        println!("???");
        return Ok(());
    }

    let cli = Cli::parse();

    if let Some(action) = cli.sub_action {
        match action {
            SubAction::Network { network_action } => match network_action {
                NetworkAction::Login { account, password } => {
                    println!("your account is {} with password {}", account, password);

                    let res = reqwest::get(format!("https://{ENTRANCE_IP}{LOGIN_NODE}?callback={LOGIN_CALLBACK}&DDDDD={account}&upass={password}&0MKKey={LOGIN_0MKKEY}")).await?;
                    println!("Status: {}", res.status());
                    println!("Headers:\n{:#?}", res.headers());

                    let body = res.text().await?;
                    let data = body.trim();
                    println!("{}", body);
                    let v: LoginResult = serde_json::from_str(&data[7..data.len() - 1])?;
                    println!("{:?}", v);
                }
                NetworkAction::Logout {} => {
                    println!("bye~");

                    let res = reqwest::get(format!(
                        "https://{ENTRANCE_IP}{LOGOUT_NODE}?callback={LOGOUT_CALLBACK}"
                    ))
                    .await?;
                    println!("Status: {}", res.status());
                    println!("Headers:\n{:#?}", res.headers());

                    let body = res.text().await?;
                    let data = body.trim();
                    println!("{}", body);
                    let v: LogoutResult = serde_json::from_str(&data[7..data.len() - 1])?;
                    println!("{:?}", v);
                }
                NetworkAction::Status { account } => {
                    let account = account.unwrap_or("null".to_string());
                    println!("your account is {}", account);

                    let res = reqwest::get(format!("https://wifi.cnu.edu.cn{QUERY_USER_INFO_NODE}?callback={QUERY_USER_INFO_CALLBACK}&account={account}")).await.unwrap();
                    println!("Status: {}", res.status());
                    println!("Headers:\n{:#?}", res.headers());

                    let body = res.text().await?;
                    let data = body.trim();
                    println!("{}", body);
                    let v: QueryUserInfoResult =
                        serde_json::from_str(&data[7..data.len() - 2])?;
                    println!("{:?}", v);

                    let res = reqwest::get(format!("https://wifi.cnu.edu.cn{QUERY_ONLINE_DEVICE_NODE}?callback={QUERY_ONLINE_DEVICE_CALLBACK}&account={account}")).await.unwrap();
                    println!("Status: {}", res.status());
                    println!("Headers:\n{:#?}", res.headers());

                    let body = res.text().await?;
                    let data = body.trim();
                    println!("{}", body);
                    let v: QueryDeviceInfoResult =
                        serde_json::from_str(&data[7..data.len() - 2])?;
                    println!("{:?}", v);
                }
            },
        }
    }

    Ok(())
}

// nmcli -g CONNECTION device status
// nmcli --show-secrets connection show W.PIE
// nmcli -g NAME connection show --active
