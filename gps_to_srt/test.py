import datetime
import json

# min_ticks = 1603793423000
# max_ticks = 1603793740000

# date_min = datetime.datetime(1970, 1, 1) + \
#     datetime.timedelta(seconds=min_ticks//1000, hours=5)

# date_max = datetime.datetime(1970, 1, 1) + \
#     datetime.timedelta(seconds=max_ticks//1000, hours=5)

# print(type(date_min.strftime("%H:%M:%S")))
# print(type(date_max-date_min))

with open("gps.json") as f:
    data = json.load(f)

for field in data["GPS"]:
    print(field["logTime"])
