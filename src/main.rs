use clap::{Parser, Subcommand};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{error::Error, process::Command};

const ENTRANCE_IP: &'static str = "wifi.cnu.edu.cn";

const LOGIN_NODE: &'static str = "/drcom/login";
const LOGIN_CALLBACK: &'static str = "ctbox";
const LOGIN_0MKKEY: &'static str = "123456";

const LOGOUT_NODE: &'static str = "/drcom/logout";
const LOGOUT_CALLBACK: &'static str = "ctbox";

const QUERY_USER_INFO_NODE: &'static str = ":802/eportal/portal/custom/loadUserInfo";
const QUERY_USER_INFO_CALLBACK: &'static str = "ctbox";
const QUERY_ONLINE_DEVICE_NODE: &'static str = ":802/eportal/portal/custom/loadOnlineDevice";
const QUERY_ONLINE_DEVICE_CALLBACK: &'static str = "ctbox";

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
    mac: Option<String>,
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

/// dr1004\({}\)
/// r"(dr1004\()(?P<content>[\s\S]*?)(\))"
fn generate_regex(template: &str) -> String {
    let re = Regex::new(r"^(?P<head>.*?)(?P<_>\{\})(?P<tail>.*?)$").unwrap();

    let head = re
        .captures(template)
        .and_then(|cap| cap.name("head").map(|head| head.as_str()))
        .unwrap();

    let tail = re
        .captures(template)
        .and_then(|cap| cap.name("tail").map(|tail| tail.as_str()))
        .unwrap();

    format!(r"({head})(?P<content>[\s\S]*?)({tail})")
}

fn fuck_cnu_api<'a>(source: &'a str, template: &'a str) -> &'a str {
    let re = Regex::new(&generate_regex(template)).unwrap();

    re.captures(source)
        .and_then(|cap| cap.name("content").map(|content| content.as_str()))
        .unwrap()
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

async fn login(account: &str, password: &str) -> Result<(), Box<dyn Error>> {
    let res = reqwest::get(format!("https://{ENTRANCE_IP}{LOGIN_NODE}?callback={LOGIN_CALLBACK}&DDDDD={account}&upass={password}&0MKKey={LOGIN_0MKKEY}")).await?;
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    let template = format!(r"{QUERY_USER_INFO_CALLBACK}\({{}}\)");
    let data = fuck_cnu_api(&body, &template);
    let v: LoginResult = serde_json::from_str(data)?;
    println!("{:?}", v);

    Ok(())
}

async fn logout() -> Result<(), Box<dyn Error>> {
    let res = reqwest::get(format!(
        "https://{ENTRANCE_IP}{LOGOUT_NODE}?callback={LOGOUT_CALLBACK}"
    ))
    .await?;
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    let template = format!(r"{QUERY_USER_INFO_CALLBACK}\({{}}\)");
    let data = fuck_cnu_api(&body, &template);
    let v: LogoutResult = serde_json::from_str(data)?;
    println!("{:?}", v);

    Ok(())
}

async fn query_user_info(account: &str) -> Result<(), Box<dyn Error>> {
    let res = reqwest::get(format!("https://wifi.cnu.edu.cn{QUERY_USER_INFO_NODE}?callback={QUERY_USER_INFO_CALLBACK}&account={account}")).await.unwrap();
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    let template = format!(r"{QUERY_USER_INFO_CALLBACK}\({{}}\);");
    let data = fuck_cnu_api(&body, &template);
    let v: QueryUserInfoResult = serde_json::from_str(data)?;
    println!("{:?}", v);

    Ok(())
}

async fn query_device_info(account: &str) -> Result<(), Box<dyn Error>> {
    let res = reqwest::get(format!("https://wifi.cnu.edu.cn{QUERY_ONLINE_DEVICE_NODE}?callback={QUERY_ONLINE_DEVICE_CALLBACK}&account={account}")).await.unwrap();
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    let template = format!(r"{QUERY_USER_INFO_CALLBACK}\({{}}\);");
    let data = fuck_cnu_api(&body, &template);
    let v: QueryDeviceInfoResult = serde_json::from_str(data)?;
    println!("{:?}", v);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if !is_cnu() {
        println!("???");
        return Ok(());
    }

    println!(
        "{}",
        fuck_cnu_api(
            r"
    dr1005({
        a
        b
        c
        d
        e
    });
    ",
            r"dr1005\({}\)"
        )
    );

    let cli = Cli::parse();

    if let Some(action) = cli.sub_action {
        match action {
            SubAction::Network { network_action } => match network_action {
                NetworkAction::Login { account, password } => {
                    println!("your account is {} with password {}", account, password);

                    login(&account, &password).await?;
                }
                NetworkAction::Logout {} => {
                    println!("bye~");

                    logout().await?;
                }
                NetworkAction::Status { account } => {
                    let account = account.unwrap_or("null".to_string());
                    println!("your account is {}", account);

                    query_user_info(&account).await?;
                    query_device_info(&account).await?;
                }
            },
        }
    }

    Ok(())
}
