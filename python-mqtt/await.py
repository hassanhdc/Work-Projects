import asyncio
from asyncio.runners import run
from asyncio.tasks import sleep


# async def add(start, end, wait):
#     sum = 0

#     for n in range(start, end):
#         sum += n
#         await asyncio.sleep(wait)

#     print(f"Sum from start {start} to {end} is {sum}")


# async def print_something():
#     for _ in range(100):
#         print("Something")
#         await asyncio.sleep(0.1)


# async def main():
#     task1 = loop.create_task(add(1, 1000000, 0))
#     task2 = loop.create_task(print_something())
#     await asyncio.wait([task1, task2])

# if __name__ == "__main__":
#     loop = asyncio.get_event_loop()
#     loop.run_until_complete(main())
#     loop.close()


async def f():
    await asyncio.sleep(1)
    return 123


async def main():
    result = await f()
    return result


if __name__ == "__main__":
    result = asyncio.run(main())

    print(result)
