use super::{
    config, data,
    network::{self, encrypt, login, logout, query},
    Cli, SubAction, This,
};
use clap::Parser;

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
                        } => login(
                            &mut data.network,
                            &config.network,
                            login_with_account,
                            login_with_label,
                        )
                        .unwrap_or_else(|e| println!("{}", e)),
                        network::Command::Logout {} => {
                            logout(&data.network, &config.network).unwrap()
                        }
                        network::Command::Query { account } => {
                            query(&data.network, &config.network, &account).unwrap()
                        }
                        network::Command::Encrypt { decrypt, source } => {
                            encrypt(&data.network, &config.network, decrypt, &source).unwrap()
                        }
                    }
                }
                SubAction::This { this_action } => match this_action {
                    This::Config {} => println!(
                        "PATH:\n{}\nCONTENT:\n{:?}",
                        config::path().to_string_lossy(),
                        config
                    ),
                    This::Data {} => println!(
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
