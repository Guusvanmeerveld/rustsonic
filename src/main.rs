mod config;
mod constants;
mod daemon;
mod keyring;
mod request;
mod utils;

use std::panic::{self, PanicInfo};

use dirs;

fn panic_hook(info: &PanicInfo<'_>) {
    let msg = match info.payload().downcast_ref::<&'static str>() {
        Some(s) => *s,
        None => match info.payload().downcast_ref::<String>() {
            Some(s) => &s[..],
            None => "Box<Any>",
        },
    };

    println!(
        "An error occurred at the following step:\n{}\n at {}",
        msg,
        info.location().unwrap(),
    )
}

#[tokio::main]
async fn main() -> utils::Result<()> {
    panic::set_hook(Box::new(panic_hook));

    let config_locations: Vec<String> = vec![
        format!(
            "{}/{}/config.toml",
            dirs::config_dir().unwrap().display(),
            constants::APPLICATION_NAME
        ),
        String::from("./config.toml"),
    ];

    let config = config::read_config(config_locations);

    if config.daemon {
        daemon::start_daemon();
    }

    let api = request::Api { config: &config };

    println!("{}", api.ping().await);

    Ok(())
}
