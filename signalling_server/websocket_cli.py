import asyncio
import websockets
from aioconsole import ainput

PEER_ID = b'1234'


async def hello(websocket):

    await websocket.send(b'Hello ' + PEER_ID)

    hello = await websocket.recv()
    assert(hello.decode('utf-8') == "Hello")
    print("Connection established with server")


async def get_input(websocket):
    while True:
        msg = await ainput()
        await websocket.send(msg.encode('utf-8'))


async def listener(websocket):
    async for message in websocket:
        print(f"Message received : {message.decode('utf-8')}")


async def main():
    uri = "ws://localhost:8765"
    async with websockets.connect(uri) as websocket:
        await hello(websocket)
        print("Enter message to send\n")
        loop = asyncio.get_event_loop()
        loop.create_task(listener(websocket))
        loop.create_task(get_input(websocket))
        await websocket.wait_closed()


asyncio.get_event_loop().run_until_complete(main())
asyncio.get_event_loop().run_forever()
