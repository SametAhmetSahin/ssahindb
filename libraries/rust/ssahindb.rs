use std::io::prelude::*;
use std::net::{TcpStream};



pub struct Client {
    address: String,
}

impl Client {
    pub fn open(address: String) -> Client {
        return Client {
            address,
        };
    }
    pub fn get_connection(&self) -> Connection {
        return Connection {
            address: self.address.clone(),
        };
    }
}

pub struct Connection {
    address: String,
}
impl Connection {
    pub fn get(&self, key: String) -> String {
        self.send(format!("get {}", key)).expect("Couldn't send command")
    }
    pub fn set(&self, key: String, val: String) -> String {
        self.send(format!("set {} {}", key, val)).expect("Couldn't send command")
    }
    pub fn del(&self, key: String) -> String {
        self.send(format!("del {}", key)).expect("Couldn't send command")
    }
    /*
    pub fn del(keys: Vec<String>) -> String {
        let mut command = "del";
        for key in keys {
            command += " " + key;
        }
        self.send(command)
    }
    */
    pub fn exists(&self, key: String) -> String {
        self.send(format!("exists {}", key)).expect("Couldn't send command")
    }

    fn send(&self, command: String) -> Result<String, String> {

        let input = command;
        match TcpStream::connect(self.address.clone()) {
            Ok(mut stream) => {
                //println!("input as bytes: {:?}", input.as_bytes());
                stream.write(input.as_bytes()).expect("Couldn't write to stream");
                //stream.flush(); 
                let mut resbuffer = vec![0 as u8; 512];
                stream.read(&mut resbuffer).expect("Couldn't read from stream");
                

                stream.shutdown(std::net::Shutdown::Both).expect("Couldn't shut down the connection");
                Ok(String::from_utf8_lossy(&resbuffer).replace("\0", ""))
            },
            Err(_) => {
                Err(format!("An error occured while trying to connect to {}.", self.address))
            }
        }
    }
}



