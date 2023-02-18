import ssahindb

con = ssahindb.connection("localhost:50394")

print(con.exists("key3"))

print(con.set("key3", "val3"))

print(con.exists("key3"))

print(con.get("key3"))

print(con.delete("key3"))

print(con.exists("key3"))

