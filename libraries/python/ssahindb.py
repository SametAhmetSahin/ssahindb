
import socket
from time import sleep

def send(ip: str, port: int, data: str):
    s = socket.socket()

    while True:
        try:
            s.connect((ip, port))
            s.send(data.encode())
            return s.recv(8192).decode()
        except socket.error as e:
            print(f"{e}! Retrying in 3 seconds...")
            sleep(3)
class connection:

    ip = ""
    port = 50394

    def __init__(self, address: str):
        self.ip = address.split(":")[0]
        self.port = int(address.split(":")[1])

    def set(self, key, value):
        return send(self.ip, self.port, f"set {key} {value}")
        pass

    def get(self, key):
        return send(self.ip, self.port, f"get {key}")
        pass

    def delete(self, key):
        return send(self.ip, self.port, f"del {key}")
        pass

    def exists(self, key):
        return send(self.ip, self.port, f"exists {key}")
        pass

    def test(self):
        return "it works!"