use crate::error::Error;
use crate::Result;
use crate::{error::Kind, network};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    result: i32,
    v46ip: String,
    olmac: String,
    uid: String,
}

pub fn login(account: &str, password: &str) -> Result<Response> {
    const NODE: &'static str = "/drcom/login";
    const CALLBACK: &'static str = env!("CARGO_PKG_NAME");
    const KEY: &'static str = "123456";

    let mut url = url::Url::parse(network::ENTRANCE)
        .unwrap()
        .join(NODE)
        .unwrap();
    let url = url
        .query_pairs_mut()
        .append_pair("callback", CALLBACK)
        .append_pair("DDDDD", account)
        .append_pair("upass", password)
        .append_pair("0MKKey", KEY)
        .finish();

    reqwest::blocking::get(url.as_str()).map_or_else(
        |_| Err(Error::new(Kind::Request)),
        |res| -> Result<Response> {
            if res.status() != 200 {
                Err(Error::with_detail(
                    Kind::Request,
                    Some(res.status().as_u16()),
                    Some(res.text().map_err(|_| Error::new(Kind::Request))?),
                ))
            } else {
                let template = format!(r"{CALLBACK}\({{}}\)");
                let source = res.text().map_err(|_| Error::new(Kind::Request))?;
                let json = network::util::fuck_cnu_api(&source, &template);

                serde_json::from_str(json).map_err(|_| Error::new(Kind::Parse))
            }
        },
    )
}
