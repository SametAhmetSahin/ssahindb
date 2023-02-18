use std::fs::File;
use std::fs;
use std::io::Write;
use std::process::exit;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};


fn main() {
    println!("SSahinDB starting...");

    //set_key("test1".to_string(), "test1".to_string());

    //println!("{}", get_key("test1".to_string()));
    start_listener().expect("Couldn't start server");
}

fn start_listener() -> std::io::Result<()> {
    
    let listener = TcpListener::bind("0.0.0.0:50394")?;
    
    for stream in listener.incoming() {
        handle_client(stream?)
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    let bufsize = 8192;
    let mut buf = vec![0 as u8; bufsize+1];
    match stream.read(&mut buf) {
        Ok(_) => {
            let strbuf = String::from_utf8_lossy(&buf);
            //println!("strbuf {}", strbuf);
            //println!("buf {:?}", &buf);
            let replaced = strbuf.replace("\0", "");
            let split: Vec<&str> = replaced.split(" ").collect();
            let res = handle_command(split);
            //println!("res is: {}", res);
            stream.write(res.as_bytes()).expect("Couldn't write to result buffer!");
            //println!("splitvec {:?}", split);
            //stream.shutdown(std::net::Shutdown::Both).expect("Couldn't shut down the connection");
            //buf = vec![0 as u8; bufsize+1];
            
        },
        Err(_) => {
            println!("An error occured!");
            
            exit(1)
        }
    };

    
    
}

fn handle_command(args: Vec<&str>) -> String {
    println!("it is {}", args[0]);
    match args[0] {
         "get" => {
            return get_key(args[1].to_string()).to_string();
         },
         "set" => {
            if args.len() != 3 {
                return String::from("Invalid number of arguments!")
            }

            return set_key(args[1].to_string(), args[2].to_string());
         },
         "del" => {
            delete_key(args[1].to_string())
         },
         "exists" => {
            key_exists(args[1].to_string()).to_string()
         },
         _ => {

            println!("invalid command!");
            return format!("Invalid command {}, with args {:?}!", args[0], &args[1..args.len()]);
         },

    }
}

fn write_file(path: String, content: String) -> std::io::Result<()> {

    let mut file = File::create(path)?;
    write!(file, "{}", content)
}


fn set_key(key: String, value: String) -> String {
    match write_file(String::from("db/") + &key, value) {
        Ok(_) => {
            //format!("{}", key)
            String::from("OK")
        },
        Err(e) => {format!("{}", e)}
    }
}

fn key_exists(key: String) -> bool {
    return file_exists(String::from("db/") + &key)
}

fn file_exists(path: String) -> bool {

    match fs::metadata(&path) {
        Ok(_) => { true },
        Err(_) => { false }
    }

}

fn delete_file(path: String) -> std::io::Result<()> {
    match fs::remove_file(path) {
        Ok(_) => { Ok(()) },
        Err(e) => { Err(e) }
    }
}

fn delete_key(key: String) -> String {
    if key_exists(key.clone()) {

    match delete_file(String::from("db/") + &key) {
        Ok(_) => {
            //format!("{}", key)
            1.to_string()
        },
        Err(e) => {format!("{}", e)}
    }

    }
    else {
        
        0.to_string()
    }
}

fn read_file(path: String) -> Result<String, std::io::Error> {

    println!("read file path is: {}", path);
    if file_exists(path.clone()) {
        match fs::read_to_string(path) {
            Ok(contents) => { println!("Contents: {}", contents); Ok(contents) },
            Err(e) => {Err(e)}
        }
    }
    else {
        Ok("nil".to_string())
    }
    
}


fn get_key(key: String) -> String {
    return read_file(String::from("db/") + &key).expect(&("Couldn't get key ".to_string() + &key))
}