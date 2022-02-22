mod config;
mod constants;
mod keyring;
mod request;
mod utils;

use std::panic::{self, PanicInfo};

fn panic_hook(info: &PanicInfo<'_>) {
    let msg = match info.payload().downcast_ref::<&'static str>() {
        Some(s) => *s,
        None => match info.payload().downcast_ref::<String>() {
            Some(s) => &s[..],
            None => "Box<Any>",
        },
    };

    println!("An error occurred at the following step:\n{}", msg)
}

#[tokio::main]
async fn main() -> utils::Result<()> {
    panic::set_hook(Box::new(panic_hook));

    let config = config::read_config(vec!["./config.toml"]);

    let api = request::Api { config: &config };

    println!("{}", api.ping().await);

    Ok(())
}
