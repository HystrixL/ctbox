use crate::error::{Error, Kind};
use crate::network::{self, entity};
use crate::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct UserResponse {
    code: String,
    data: Vec<entity::User>,
    msg: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct DeviceResponse {
    code: String,
    data: Vec<entity::Device>,
    msg: String,
}

pub fn query_user_info(account: Option<&str>) -> Result<Vec<entity::User>> {
    const PORT: u16 = 802;
    const NODE: &str = "/eportal/portal/custom/loadUserInfo";
    const CALLBACK: &str = env!("CARGO_PKG_NAME");

    let mut url = url::Url::parse(network::ENTRANCE).unwrap();
    url.set_port(Some(PORT)).unwrap();
    let mut url = url.join(NODE).unwrap();
    let url = url
        .query_pairs_mut()
        .append_pair("callback", CALLBACK)
        .append_pair("account", account.unwrap_or("null"))
        .finish();

    reqwest::blocking::get(url.as_str()).map_or(Err(Error::new(Kind::Request)), |res| {
        if res.status() != 200 {
            Err(Error::with_detail(
                Kind::Request,
                Some(res.status().as_u16()),
                res.text().ok(),
            ))
        } else {
            let template = format!(r"{CALLBACK}\({{}}\)");
            let source = res.text().map_err(|_| Error::new(Kind::Request))?;
            let json = network::util::fuck_cnu_api(&source, &template);

            serde_json::from_str(json).map_or_else(
                |_| Err(Error::new(Kind::Parse)),
                |info: UserResponse| {
                    if info.code != "1" {
                        Err(Error::with_detail(
                            Kind::Query,
                            Some(info.code.parse::<u16>().unwrap()),
                            Some(info.msg),
                        ))
                    } else {
                        Ok(info.data)
                    }
                },
            )
        }
    })
}

pub fn query_device_info(account: Option<&str>) -> Result<Vec<entity::Device>> {
    const PORT: u16 = 802;
    const NODE: &str = "/eportal/portal/custom/loadOnlineDevice";
    const CALLBACK: &str = env!("CARGO_PKG_NAME");

    let mut url = url::Url::parse(network::ENTRANCE).unwrap();
    url.set_port(Some(PORT)).unwrap();
    let mut url = url.join(NODE).unwrap();
    let url = url
        .query_pairs_mut()
        .append_pair("callback", CALLBACK)
        .append_pair("account", account.unwrap_or(""))
        .finish();

    reqwest::blocking::get(url.as_str()).map_or(Err(Error::new(Kind::Request)), |res| {
        if res.status() != 200 {
            Err(Error::with_detail(
                Kind::Request,
                Some(res.status().as_u16()),
                res.text().ok(),
            ))
        } else {
            let template = format!(r"{CALLBACK}\({{}}\)");
            let source = res.text().map_err(|_| Error::new(Kind::Request))?;
            let json = network::util::fuck_cnu_api(&source, &template);

            serde_json::from_str(json).map_or_else(
                |_| Err(Error::new(Kind::Parse)),
                |info: DeviceResponse| {
                    if info.code != "1" {
                        Err(Error::with_detail(
                            Kind::Query,
                            Some(info.code.parse::<u16>().unwrap()),
                            Some(info.msg),
                        ))
                    } else {
                        Ok(info.data)
                    }
                },
            )
        }
    })

    /*
    if query_device_info_result.code == "1" {
        println!(
            "{:<21}{:<10}{:<13}{:<10}",
            "登录时间", "认证服务器", "设备IP", "设备MAC"
        );

        query_device_info_result.data.iter().for_each(|device| {
            println!(
                "{:<25}{:<15}{:<15}{:<12}",
                format!("{}", device.login_time),
                format!("{}", device.bas_id),
                format!("{}", device.login_ip),
                format!("{}", device.mac_address)
            )
        });
    } else {
        println!(
            "设备信息获取失败。\n状态码: {}\n错误信息: {}",
            query_device_info_result.code, query_device_info_result.msg
        );
    } */
}
