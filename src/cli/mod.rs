use clap::{Parser, Subcommand};

pub mod entrance;
pub mod network;

trait Display<T> {
    fn display(v: T);
}

#[derive(Parser)]
#[command(version, author, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub sub_action: Option<SubAction>,
}

#[derive(Subcommand)]
pub enum SubAction {
    /// 校园网相关操作
    Network {
        #[command(subcommand)]
        network_action: network::Command,
    },
}
