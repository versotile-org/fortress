//! FORTRESS main application

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(about = include_str!("../logo.ascii"))]
pub struct Fortress {
    #[clap(short, long, required = false)]
    verbose: bool,

    #[clap(
        short,
        long,
        help = "Provide the path to the STORAGE file. Default is /path/to/user/STORAGE.json",
        default_value = "STORAGE.json",
        required = false
    )]
    storage: PathBuf,

    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Subcommand, Debug, Clone)]
#[command(arg_required_else_help = true)]
pub enum Command {
    #[clap(about = "Fetch a data entry")]
    Fetch {
        #[clap(short, long)]
        key: String,
    },

    #[clap(about = "Stores a new data entry")]
    Store,

    #[clap(about = "Sign a <file> with given <token> to a private key with selected <algorithm>")]
    Sign {
        #[clap(short, long, help = "Signature algorithm to use")]
        algorithm: String,

        #[clap(short, long, help = "The token to access the private key")]
        token: String,

        #[clap(short, long, help = "The file to sign")]
        file: PathBuf,
    },

    #[clap(about = "Create a new signature keypair. Returns a token to access the private key")]
    Genkey {
        #[clap(short, long, help = "The algorithm to use for keypair generation")]
        algorithm: String,

        #[clap(
            short,
            long,
            help = "Token expire date. The `genkey` command will create an access token with an attached lifetime. "
        )]
        token_expire: String,
    },

    #[clap(about = "Runs the FORTRESS daemon.")]
    Daemon,
}

fn main() {
    let _ = Fortress::parse();
}
