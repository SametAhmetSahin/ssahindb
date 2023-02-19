use clap::Parser;
use std::fs::File;
use std::fs;
use std::io::Write;
use std::process::exit;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

struct Args {
    #[arg(short, long, default_value_t=String::from("0.0.0.0:50394"))]
    address: String, //Option<String> // in format of ip:port

    #[arg(short, long, default_value_t=String::from("m"))]
    mode: String, // should values be written to disk? Possible values are m/memory, d/disk, b/both

}


fn main() {
    println!("SSahinDB starting...");

    let args = Args::parse();

    let mut db: HashMap<String, String> = HashMap::new();

    start_listener(args, &mut db).expect("Couldn't start server");
}

fn start_listener(args: Args, db: &mut HashMap<String, String>) -> std::io::Result<()> {
    
    let listener = TcpListener::bind(&args.address)?;

    println!("Listening at {}", &args.address);
    
    for stream in listener.incoming() {
        handle_client(&args, db, stream?)
    }
    Ok(())
}

fn handle_client(args: &Args, db: &mut HashMap<String, String>, mut stream: TcpStream) {
    let bufsize = 8192;
    let mut buf = vec![0 as u8; bufsize+1];
    match stream.read(&mut buf) {
        Ok(_) => {
            let strbuf = String::from_utf8_lossy(&buf);
            //println!("strbuf {}", strbuf);
            //println!("buf {:?}", &buf);
            let replaced = strbuf.replace("\0", "");
            let split: Vec<&str> = replaced.split(" ").collect();//(&replaced[0..&replaced.len()-1]).split(" ").collect();
            let res = handle_command(args, split, db);
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

fn handle_command(args: &Args, command_args: Vec<&str>, db: &mut HashMap<String, String>) -> String {
    //println!("it is {}", args[0]);
    
    let command_args_count = HashMap::from([
        ("get", 2),
        ("set", 3),
        ("del", 2),
        ("exists", 2),
    ]);
    
    if command_args_count.contains_key(command_args[0]) {
        if command_args_count.get(command_args[0]) != Some(&command_args.len()) {
            return format!("Invalid number of args for command {}, with args {:?}! There should be {} argument(s)", command_args[0], &command_args[1..command_args.len()], command_args_count.get(&command_args[0]).unwrap()-1);
        }
    }

    match command_args[0] {
         "get" => {
            get_key(args, db, command_args[1].to_string()).to_string()
         },
         "set" => {
            set_key(args, db, command_args[1].to_string(), command_args[2].to_string())
         },
         "del" => {
            delete_key(args, db, command_args[1].to_string())
         },
         "exists" => {
            key_exists(args, db, command_args[1].to_string()).to_string()
         },
         _ => {

            //println!("invalid command!");
            return format!("Invalid command {}, with args {:?}!", command_args[0], &command_args[1..command_args.len()]);
         },

    }
}

fn write_file(path: String, content: String) -> std::io::Result<()> {

    let mut file = File::create(path)?;
    write!(file, "{}", content)
}


fn set_key(args: &Args, db: &mut HashMap<String, String>, key: String, value: String) -> String {

    
    let mut res = String::from("");
    if vec!["m", "memory", "b", "both"].contains(&args.mode.as_str()) {
        db.insert(key.clone(), value.clone());
        res = String::from("OK");
    }
    if vec!["d", "disk", "b", "both"].contains(&args.mode.as_str()) {
        
        match fs::metadata("db") {
            Ok(_) => { 
    
            },
            Err(_) => { 
                fs::create_dir("db").expect("Couldn't create directory db");
            }
        }
    
        match write_file(String::from("db/") + &key, value) {
            Ok(_) => {
                //format!("{}", key)
                res = String::from("OK")
            },
            Err(e) => {res = format!("{}", e)}
        }
    }
    return res;
    
}

fn key_exists(args: &Args, db: &HashMap<String, String>, key: String) -> bool {

    let mut res: bool = false;
    if vec!["m", "memory", "b", "both"].contains(&args.mode.as_str()) {
        res = db.contains_key(&key);
    }
    if vec!["d", "disk", "b", "both"].contains(&args.mode.as_str()) {
        res = file_exists(String::from("db/") + &key);
    }
    return res
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

fn delete_key(args: &Args, db: &mut HashMap<String, String>, key: String) -> String {

    println!("Attemted to delete key {}", &key);

    let mut res = String::from("0");

        if vec!["m", "memory", "b", "both"].contains(&args.mode.as_str()) {
            if db.contains_key(&key) {
                db.remove(&key);
                res = 1.to_string();
            }
        }

        if vec!["d", "disk", "b", "both"].contains(&args.mode.as_str()) {

            if file_exists(String::from("db/") + &key) {
            
                match delete_file(String::from("db/") + &key) {
                    Ok(_) => {
                        //format!("{}", key)
                        res = 1.to_string();
                    },
                    Err(e) => {res = format!("{}", e)}
                }
            }
        }
        return res
}

fn read_file(path: String) -> Result<String, std::io::Error> {

    //println!("read file path is: {}", path);
    if file_exists(path.clone()) {
        match fs::read_to_string(path) {
            Ok(contents) => { Ok(contents) },
            Err(e) => {Err(e)}
        }
    }
    else {
        Ok("nil".to_string())
    }
    
}


fn get_key(args: &Args, db: &HashMap<String, String>, key: String) -> String {
    let mut res = String::from("nil");
    if vec!["m", "memory", "b", "both"].contains(&args.mode.as_str()) {
        if db.contains_key(&key) {
        res = db.get(&key).unwrap().to_string(); // Using unwrap because if the key doesn't exist in won't enter into the if in the first place
        }
    }
    if vec!["d", "disk", "b", "both"].contains(&args.mode.as_str()) {
        res = read_file(String::from("db/") + &key).expect(&("Couldn't get key ".to_string() + &key))
    }
    return res;
}