use clap::{Parser, Subcommand};

pub mod network;
pub mod format;

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