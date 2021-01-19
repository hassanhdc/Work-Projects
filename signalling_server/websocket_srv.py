# import socket
import asyncio
import websockets
from websockets.exceptions import ConnectionClosedError

connected = set()
peers = dict()


async def hello_peer(websocket):
    raddr = websocket.remote_address
    hello = await websocket.recv()
    hello, peer_id = hello.split(maxsplit=1)
    await websocket.send(b'Hello')
    return peer_id


async def connection_handler(websocket, peer_id):
    raddr = websocket.remote_address

    peers[peer_id] = [websocket, raddr]

    print(f"Connection established with client {peer_id}")


async def sender(websocket, message):
    try:
        await websocket.send(message)
    except ConnectionClosedError:
        print("Client has disconnected. Unable to send message")


async def listener(websocket):
    raddr = websocket.remote_address
    try:
        async for message in websocket:
            print(f"Message received : {message.decode('utf-8')}")
    except ConnectionClosedError:
        print(
            f"Client has disconnected. Unable to listen to message from {raddr}\n")


async def hello_repeat(websocket):
    message = b'Hello'
    while True:
        await sender(websocket, message)
        await asyncio.sleep(3)


async def server(websocket, _):
    raddr = websocket.remote_address
    print("Connected to {!r}".format(raddr))
    peer_id = await hello_peer(websocket)
    try:
        await connection_handler(websocket, peer_id)
    except ConnectionClosedError:
        print(f"Connection to peer {raddr} closed")

    loop = asyncio.get_event_loop()
    loop.create_task(listener(websocket))
    loop.create_task(hello_repeat(websocket))

    await websocket.wait_closed()

start_server = websockets.serve(server, "localhost", 8765)

try:
    asyncio.get_event_loop().run_until_complete(start_server)
    asyncio.get_event_loop().run_forever()
except KeyboardInterrupt:
    print("Connection Closed..Exiting")
