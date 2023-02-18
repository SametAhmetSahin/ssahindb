use std::io;
use std::io::prelude::*;
use std::net::{TcpStream};

fn main() -> std::io::Result<()> {

    let address = "0.0.0.0:50394";

    loop {

    let mut lock = io::stdout().lock();

    write!(lock, "{}> ", address).expect("Couldn't write to terminal");
    //print!("{}> ", address);

    io::stdout().flush().expect("Couldn't flush stdout");

    let mut input = String::from("");
    io::stdin().read_line(&mut input).expect("Couldn't read input");

    //println!("Sending message:\n{}", &input);

        if input != "\n" {
            while !match TcpStream::connect(address) {
                Ok(mut stream) => {
                    //println!("input as bytes: {:?}", input.as_bytes());
                    stream.write((&input[0..input.len()-1]).as_bytes())?;
                    //stream.flush(); 
                    let mut resbuffer = vec![0 as u8; 512];
                    stream.read(&mut resbuffer);
                    
                    //println!("resbuffer: {:?}", resbuffer);

                    println!("{:?}", String::from_utf8_lossy(&resbuffer).replace("\0", ""));

                    stream.shutdown(std::net::Shutdown::Both).expect("Couldn't shut down the connection");
                    true
                },
                Err(_) => {
                    println!("An error occured while trying to connect to {}. Trying again in 3 seconds.", address);
                    
                    std::thread::sleep(std::time::Duration::from_secs(3));
                    false
                }
            } {}
        }

    }
}