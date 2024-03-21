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
    Query {
        /// 账号
        #[arg(short, long)]
        account: Option<String>,
    },
    Encrypt {
        #[arg(short, long)]
        decrypt: bool,
        source: String,
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
    out.split_whitespace().any(|v| v == "CNU")
}

async fn login(account: &str, password: &str) -> Result<(), Box<dyn Error>> {
    let res = reqwest::get(format!("https://{ENTRANCE_IP}{LOGIN_NODE}?callback={LOGIN_CALLBACK}&DDDDD={account}&upass={password}&0MKKey={LOGIN_0MKKEY}")).await?;
    if res.status() != 200 {
        println!(
            "登录失败。\n状态码: {}\n错误信息: {}",
            res.status(),
            res.text().await?
        );
        return Ok(());
    }

    let body = res.text().await?;
    let template = format!(r"{QUERY_USER_INFO_CALLBACK}\({{}}\)");
    let data = fuck_cnu_api(&body, &template);
    let login_result: LoginResult = serde_json::from_str(data)?;

    if login_result.result != 1 {
        println!("登录失败。\n状态码: {}", login_result.result);
        return Ok(());
    }

    println!(
        "{} 登录成功。\n本机IP: {}\n余额: {}\n设备数量: {}",
        login_result.uid, "?", "?", "?"
    );
    Ok(())
}

async fn logout() -> Result<(), Box<dyn Error>> {
    let res = reqwest::get(format!(
        "https://{ENTRANCE_IP}{LOGOUT_NODE}?callback={LOGOUT_CALLBACK}"
    ))
    .await?;
    if res.status() != 200 {
        println!(
            "登出失败。\n状态码: {}\n错误信息: {}",
            res.status(),
            res.text().await?
        );
        return Ok(());
    }

    let body = res.text().await?;
    let template = format!(r"{QUERY_USER_INFO_CALLBACK}\({{}}\)");
    let data = fuck_cnu_api(&body, &template);
    let logout_result: LogoutResult = serde_json::from_str(data)?;
    if logout_result.result != 1 {
        println!("登出失败。\n状态码: {}", logout_result.result);
        return Ok(());
    }
    println!("登出成功。");
    Ok(())
}

async fn query_user_info(account: &str) -> Result<(), Box<dyn Error>> {
    let res = reqwest::get(format!("https://wifi.cnu.edu.cn{QUERY_USER_INFO_NODE}?callback={QUERY_USER_INFO_CALLBACK}&account={account}")).await.unwrap();

    if res.status() != 200 {
        println!(
            "用户信息获取失败。\n状态码: {}\n错误信息: {}",
            res.status(),
            res.text().await?
        );
        return Ok(());
    }

    let body = res.text().await?;
    let template = format!(r"{QUERY_USER_INFO_CALLBACK}\({{}}\);");
    let data = fuck_cnu_api(&body, &template);
    let query_user_info_result: QueryUserInfoResult = serde_json::from_str(data)?;

    println!("{account}的校园网信息");
    if query_user_info_result.code == "1" {
        let user_infos = &query_user_info_result.data;
        println!(
            "{:<21}{:<11}{:<11}{:<9}",
            "已用流量", "已用时长", "用户余额", "无感知MAC"
        );

        for user_info in user_infos {
            println!(
                "{:<25}{:<15}{:<14}{:<12}",
                format!("{}MB", user_info.user_flow),
                format!("{}Min", user_info.user_time),
                format!("{}元", user_info.user_money),
                format!("{}", user_info.mac.as_ref().unwrap_or(&"无".to_string()))
            );
        }
    } else {
        println!(
            "用户信息获取失败。\n状态码: {}\n错误信息: {}",
            query_user_info_result.code, query_user_info_result.msg
        );
    }

    Ok(())
}

async fn query_device_info(account: &str) -> Result<(), Box<dyn Error>> {
    let res = reqwest::get(format!("https://wifi.cnu.edu.cn{QUERY_ONLINE_DEVICE_NODE}?callback={QUERY_ONLINE_DEVICE_CALLBACK}&account={account}")).await.unwrap();
    if res.status() != 200 {
        println!(
            "设备信息获取失败。\n状态码: {}\n错误信息: {}",
            res.status(),
            res.text().await?
        );
        return Ok(());
    }
    let body = res.text().await?;
    let template = format!(r"{QUERY_USER_INFO_CALLBACK}\({{}}\);");
    let data = fuck_cnu_api(&body, &template);
    let query_device_info_result: QueryDeviceInfoResult = serde_json::from_str(data)?;
    if query_device_info_result.code == "1" {
        let devices = &query_device_info_result.data;
        println!(
            "{:<21}{:<10}{:<13}{:<10}",
            "登录时间", "认证服务器", "设备IP", "设备MAC"
        );

        for device in devices {
            println!(
                "{:<25}{:<15}{:<15}{:<12}",
                format!("{}", device.login_time),
                format!("{}", device.bas_id),
                format!("{}", device.login_ip),
                format!("{}", device.mac_address)
            );
        }
    } else {
        println!(
            "设备信息获取失败。\n状态码: {}\n错误信息: {}",
            query_device_info_result.code, query_device_info_result.msg
        );
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if !is_cnu() {
        println!("???");
        return Ok(());
    }

    let cli = Cli::parse();

    if let Some(action) = cli.sub_action {
        match action {
            SubAction::Network { network_action } => match network_action {
                NetworkAction::Login { account, password } => {
                    login(&account, &password).await?;
                }
                NetworkAction::Logout {} => {
                    logout().await?;
                }
                NetworkAction::Query { account } => {
                    let account = account.unwrap_or("null".to_string());
                    query_user_info(&account).await?;
                    println!();
                    query_device_info(&account).await?;
                }
                NetworkAction::Encrypt { decrypt, source } => {
                    let first_key = "689abcrstu%012345vwxyABCDEFGdefghMNOPQRijklmnpqHIJKSTUVWXYZ";
                    let second_key = "rsHYZ23tFhiIJjku9abP5QRScABd8DVWXElmGvwK%01xyC4npqMgNOTU6ef";
                    let (first_key, second_key) = if decrypt {
                        (second_key, first_key)
                    } else {
                        (first_key, second_key)
                    };

                    let mut result = String::new();
                    for (index, value) in source.chars().enumerate() {
                        if let Some(index_in_first) = first_key.chars().position(|x| x == value) {
                            let mut index_in_first = index_in_first as i32;
                            index_in_first += if decrypt {
                                index as i32
                            } else {
                                -(index as i32)
                            };
                            index_in_first %= first_key.len() as i32;
                            if index_in_first < 0 {
                                index_in_first += first_key.len() as i32;
                            }
                            let index_in_first = index_in_first as usize;
                            result.push_str(&second_key[index_in_first..index_in_first + 1]);
                        } else {
                            result.push(source.chars().last().unwrap());
                        }
                    }
                    println!("{result}");
                }
            },
        }
    }

    Ok(())
}
