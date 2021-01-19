import asyncio
import websockets

PEER_ID = b'12345'


async def cli():
    uri = "ws://localhost:8765"
    async with websockets.connect(uri) as websocket:
        await websocket.send(PEER_ID)
        await websocket.send(b'Hello')

        hello = await websocket.recv()
        assert(hello.decode('utf-8') == "Hello")
        print("Connection established with server")


asyncio.get_event_loop().run_until_complete(cli())
