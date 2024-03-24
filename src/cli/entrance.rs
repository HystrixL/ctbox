use super::{
    config, data,
    network::{self, LoginWithAccount},
    Cli, SubAction,
};
use clap::Parser;

impl Cli {
    pub fn process() {
        let cli = Self::parse();

        if let Some(action) = cli.sub_action {
            let data = match data::read() {
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
                    if !ctbox::network::util::is_cnu() {
                        eprintln!("无法访问校园网入口，请检查您及校园网的网络状态。");
                        return;
                    };
                    match network_action {
                        network::Command::Login {
                            login_with_account,
                            login_with_label: _,
                        } => {
                            if let LoginWithAccount {
                                account: Some(a),
                                password: Some(p),
                                save: None,
                                default: false,
                            } = login_with_account
                            {
                                network::login(&a, &p, true);
                            } else {
                                println!("功能未完成! 敬请期待.");
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
            }
        }
    }
}
