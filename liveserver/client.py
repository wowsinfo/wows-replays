import asyncio
from websockets.sync.client import connect
import time

def test():
    with connect("ws://127.0.0.1:8615") as websocket:
        # send 5 messages before closing
        for i in range(5):
            websocket.send(f"Hello {i}!")
            message = websocket.recv()
            print(f"Received: {message}")

if __name__ == "__main__":
    test() 
