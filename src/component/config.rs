use clap::Parser;
use std::borrow::Cow;
use std::net::SocketAddr;

#[derive(Debug, Parser)]
#[command(author, version, long_about = None)]
pub struct Config {
	#[arg(short, long, default_value = "127.0.0.1:4444")]
	pub address: SocketAddr,
	#[arg(short, long, default_value = "./config.xurl")]
	pub urls_config: Cow<'static, str>,
}

