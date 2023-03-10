# SSahinDB
A Redis-inspired key-value database, written in Rust.

## About
In my journey of learning Rust I'm writing small and simple clones of existing products, and this is a simple key-value database.

At the time of writing (19/02/2023 in dd/mm/yy format) I am still new to Rust, and I do not claim that the code is optimal. There are probably lots of improvements to be made.

## Installation & Running
After running `cargo build`, preferably with the `--release` flag, the `ssahindb` binary (the engine) can be run directly. By default it'll listen on the port 50394, and the address can be specified with the `-a/--address` flag. If you run the binary executable with the `--help` flag, it will display a nice dialogue generated by Clap (the command line argument parser this project uses) and show you all the flags.  
The flags are listed below, directly copied from the dialogue itself.

## Engine Usage

```
Usage: ssahindb [OPTIONS]

Options:
  -a, --address <ADDRESS>  [default: 0.0.0.0:50394]
  -m, --mode <MODE>        [default: m]
  -h, --help               Print help
  -V, --version            Print version
```

There are 3 modes for the engine, `m/memory` uses a hashmap to store values in memory, `d/disk` directly writes values to files named after their respective keys to a db directory in the working directory, and `b/both` does both. I might look into a more effective method for the disk mode.  

After executing the engine executable, it begins to listen on the specified address.

## ssahindb-cli Usage

```
Usage: ssahindb-cli [OPTIONS]

Options:
  -a, --address <ADDRESS>  [default: localhost:50394]
  -h, --help               Print help
  -V, --version            Print version
```

After executing the cli binary, the user is greeted with a prompt printing the currently connected address just like redis-cli. The user can type commands like `set key1 val1` or `get key1` or `exists key1` and so on.  
If the command or its arguments are invalid, the cli will display a warning such as `"Invalid command g, with args [\"key1\", \"val1\"]!"`, and if the argument count is invalid, it'll display `"Invalid number of args for command get, with args [\"key1\", \"val1\"]! There should be 1 argument(s)"`.

## Existing Commands
- Set: Sets a key's value
- Get: Retrieves a key's value
- Del: Deletes a key
- Exists: Returns true if the key exists, false otherwise

## Libraries
The libraries could be structured better or even as their own repos, maybe? No idea.  

Libraries for
- Python
- Rust  
  
are available for use, and their example uses are shown in `libraries/langhere/example.langext`, or below:  

## Python
```
import ssahindb

con = ssahindb.connection("localhost:50394")

print(con.exists("key3"))

print(con.set("key3", "val3"))

print(con.exists("key3"))

print(con.get("key3"))

print(con.delete("key3"))

print(con.exists("key3"))

```

## Rust
```
pub mod ssahindb;

fn main() {
    let client = ssahindb::Client::open("localhost:50394".to_string());
    let con = client.get_connection();

    println!("Key3 exists: {}", con.exists("key3".to_string()));
    con.set("key3".to_string(), "val3".to_string());

    println!("get key3: {}", con.get("key3".to_string()));
    con.del("key3".to_string());

    println!("Key3 exists: {}", con.exists("key3".to_string()));

    println!("get key3: {}", con.get("key3".to_string()));
}
```

## Known Bugs & Issues
None as far as I know. 

## What to Do
No idea at the moment. Implementing other data types found in Redis like Lists, Sets etc. could be done but I currently don't aim to as I plan this project to be very simple.