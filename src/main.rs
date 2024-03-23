mod cli;

use clap::Parser;
use cli::{Cli, SubAction};
use ctbox::network;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    if !network::util::is_cnu() {
        eprintln!("无法访问校园网入口，请检查您及校园网的网络状态。");
        return Ok(());
    }

    let cli = Cli::parse();

    if let Some(action) = cli.sub_action {
        match action {
            SubAction::Network { network_action } => match network_action {
                cli::network::Command::Login { account, password } => {
                    println!(
                        "{:?}",
                        ctbox::network::login::login(&account, &password).unwrap()
                    );
                }
                cli::network::Command::Logout {} => {
                    println!("{:?}", ctbox::network::logout::logout().unwrap());
                }
                cli::network::Command::Query { account } => {
                    let account = account.unwrap_or("null".to_string());
                    let result = ctbox::network::query::query_user_info(&account).unwrap();
                    for u in result {
                        println!("{:?}", u);
                    }
                    println!();
                    let result = ctbox::network::query::query_device_info(&account).unwrap();
                    for d in result {
                        println!("{:?}", d);
                    }
                }
                cli::network::Command::Encrypt { decrypt, source } => {
                    println!("{}", ctbox::network::encrypt::encrypt(decrypt, &source));
                }
            },
        }
    }

    Ok(())
}
