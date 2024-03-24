use std::default;

use super::{
    config, data,
    network::{self, LoginWithAccount, LoginWithLabel},
    Cli, SubAction,
};
use clap::Parser;
use ctbox::network::entity::User;

impl Cli {
    pub fn process() {
        let cli = Self::parse();

        if let Some(action) = cli.sub_action {
            let mut data = match data::read() {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("数据文件读取失败: {}", e);
                    return;
                }
            };
            let config = match config::read() {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("配置文件读取失败: {}", e);
                    return;
                }
            };

            match action {
                SubAction::Network { network_action } => {
                    if config.network.connect_check && !ctbox::network::util::is_cnu() {
                        eprintln!("无法访问校园网入口，请检查您及校园网的网络状态。");
                        return;
                    };
                    match network_action {
                        network::Command::Login {
                            login_with_account,
                            login_with_label,
                        } => {
                            if let LoginWithAccount {
                                account: Some(a),
                                password: Some(p),
                                save: s,
                                default: d,
                            } = login_with_account
                            {
                                network::login(&a, &p, true);
                                if let Some(label) = s {
                                    let user = User::new(a, p);
                                    data.network.users.insert(label.clone(), user);
                                    if d {
                                        data.network.default = label;
                                    }
                                }
                            } else if let LoginWithLabel { load: Some(l) } = login_with_label {
                                if !data.network.users.contains_key(&l) {
                                    println!("无法找到该标签的登入信息.");
                                }
                                network::login(
                                    &data.network.users[&l].account,
                                    &data.network.users[&l].password,
                                    true,
                                );
                            } else {
                                if data.network.default.is_empty() {
                                    println!("未设置默认登入信息.");
                                }
                                network::login(
                                    &data.network.users[&data.network.default].account,
                                    &data.network.users[&data.network.default].password,
                                    true,
                                );
                            }
                        }
                        network::Command::Logout {} => network::logout(),
                        network::Command::Query { account } => {
                            network::query_user(account.as_deref());
                            println!();
                            network::query_device(account.as_deref());
                        }
                        network::Command::Encrypt { decrypt, source } => {
                            network::encrypt(decrypt, &source);
                        }
                    }
                }
                SubAction::This { this_action } => match this_action {
                    super::This::Config {} => println!(
                        "PATH:\n{}\nCONTENT:\n{:?}",
                        config::path().to_string_lossy(),
                        config
                    ),
                    super::This::Data {} => println!(
                        "PATH:\n{}\nCONTENT:\n{:?}",
                        data::path().to_string_lossy(),
                        data
                    ),
                },
            }

            data::write(data).unwrap_or_else(|e| eprintln!("数据文件写入失败: {}", e));
        }
    }
}
