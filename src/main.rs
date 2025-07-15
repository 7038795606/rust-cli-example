use clap::{Arg, Command};

fn main() {
    let matches = Command::new("lsblk")
        .about("Lists block devices")
        .arg(
            Arg::new("device")
                .help("The device to inspect")
                .required(false)
                .index(1),
        )
        .get_matches();

    if let Some(device) = matches.get_one::<String>("device") {
        println!("Device: {}", device);
    } else {
        println!("No device provided");
    }
}
