use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    // Byte array to encode
    byte_array: String,
}

fn main() {
    let args = Cli::from_args();
    let bytes: Vec<u8> = args
        .byte_array
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|s| u8::from_str(s).unwrap())
        .collect();
    let encoding = bs58::encode(bytes).into_string();
    println!("{}", encoding);
}
