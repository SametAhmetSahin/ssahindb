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