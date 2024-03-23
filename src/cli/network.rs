use clap::Subcommand;

#[derive(Subcommand)]
pub enum Command {
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