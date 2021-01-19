import asyncio
from aioconsole import ainput
import sys

loop = asyncio.new_event_loop()


async def main():
    while True:
        print("\nHello World!")
        loop.create_task(user_input())
        await asyncio.sleep(3)


async def user_input():
    while True:
        try:
            msg = await ainput()
        except KeyboardInterrupt:
            break
        print(msg)

try:
    loop.run_until_complete(main())
except KeyboardInterrupt:
    print("Closed")
    sys.exit(0)
