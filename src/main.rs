use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error};

fn process_line(line: &str) {
    let pos = line.find(',');
    if pos.is_none() {
        return;
    }
    let parts: Vec<&str> = line[..pos.unwrap()]
        .split(|c| c == '.' || c == '/' || c == ',')
        .collect();

    let mut address: u32 = 0;
    let mask: u32 = (2u32.pow(32 - parts[4].parse::<u32>().unwrap())) - 1;
    for i in 0..(parts.len() - 1) {
        address |= (u32::from(parts[i].parse::<u8>().unwrap())) << (3 - i) * 8;
    }
    let start_address = address.to_be_bytes();
    let end_address = (address + mask).to_be_bytes();
    let startip = format!(
        "{}.{}.{}.{}",
        start_address[0].to_string(),
        start_address[1].to_string(),
        start_address[2].to_string(),
        start_address[3].to_string()
    );
    let stopip = format!(
        "{}.{}.{}.{}",
        end_address[0].to_string(),
        end_address[1].to_string(),
        end_address[2].to_string(),
        end_address[3].to_string()
    );

    println!("{},{}{}", startip, stopip, line.get(pos.unwrap()..line.len()).unwrap());
}

fn main() -> Result<(), Error> {
    let path = "/home/anders/slask/GeoLite2-Country-CSV/GeoLite2-Country-Blocks-IPv4.csv";
    let input = File::open(path).expect("file not found");
    let buffered = BufReader::new(input);
    for (index, line) in buffered.lines().enumerate() {
        if index > 0 {
            process_line(&line?);
        }
    }
    Ok(())
}
