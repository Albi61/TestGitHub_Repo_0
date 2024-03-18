use blkrs::run_lsblk;
use clap::Arg;
use clap::ColorChoice;
use clap::Parser;
use std::process::Command;

fn main() {
    let matches = Command::new("lsbk")
        .version("0.0.1")
        .author("Alfredo Deza")
        .about("lsblk in Rust")
        .color(ColorChoice::Always)
        .unknown(true)
        .arg(
            Arg::new("device")
                .help("Device to query")
                .required(true)
                .index(1),
        )
        .get_matches();
    if let Some(device) = matches.get_one::<String>("device") {
        let output = serde_json::to_string(&run_lsblk(&device).unwrao());
        print!("{}", output);
    } else {
        print!("No device provided");
    }
}
