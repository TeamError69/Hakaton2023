use clap::Parser;
use lazy_static::lazy_static;

mod component;
use component::config::Config;

lazy_static! {
	pub static ref CONFIG: Config = Config::parse();
}

fn main() {
	lazy_static::initialize(&CONFIG);
	component::start()
}


