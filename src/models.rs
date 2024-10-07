use colored::*;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(short, long)]
    pub threads: Option<usize>,
    #[clap(short, long)]
    pub address: Option<String>,
    #[clap(short, long)]
    pub pool: Option<String>,
}

impl Args {
    pub fn parse_and_validate() -> Args {
        let args = Args::parse();

        if args.address.is_none() || args.pool.is_none() {
            Args::show_demo_usage();
            std::process::exit(0);
        }

        args
    }

    pub fn show_demo_usage() {
        println!();
        println!("{}", "Run the miner with required arguments:".bold().bright_yellow());
        println!("{}", "--address <shaicoin_address> --pool <POOL_URL>".bold().bright_red());
        println!("{}", "OPTIONAL: --threads <AMT>".bold().bright_red());
        println!();
        println!("Example mining with 4 threads:");
        println!("./shaipot --address sh1qeexkz69dz6j4q0zt0pkn36650yevwc8eksqeuu --pool ws://pool.shaicoin.org --threads 4");
    }
}

#[derive(Serialize, Deserialize)]
pub struct SubmitMessage {
    pub r#type: String,
    pub miner_id: String,
    pub nonce: String,
    pub job_id: String,
    pub path: String,
}

#[derive(Deserialize, Debug)]
pub struct ServerMessage {
    pub r#type: String,
    pub job_id: Option<String>,
    pub data: Option<String>,
    pub target: Option<String>,
    pub pplns_score: Option<f64>,
}

#[derive(Clone, Debug)]
pub struct Job {
    pub job_id: String,
    pub data: String,
    pub target: String,
}