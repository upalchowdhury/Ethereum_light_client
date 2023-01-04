#[warn(unused_imports)]
use std::{
    fs,
    path::PathBuf,// cross platform path manipulation    
    process::exit,
    str::FromStr,
    sync::{Arc, Mutex},
};

use clap::Parser;
use common::utils::hex_str_to_bytes; //custom          
use dirs::home_dir;
use env_logger::Env;
use eyre::Result;

use client::{database::FileDB, Client, ClientBuilder};
use config::{CliConfig, Config};
use futures::executor::block_on;
use log::info;

#[tokio::main]
async fn main() -> Result<()> {

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = get_config();
    let mut client = ClientBuilder::new().config(config).build()?;

    client.start().await?;

    register_shutdown_handler(client);
    std::future::pending().await // keeps the functions running forever unless manually closed

}