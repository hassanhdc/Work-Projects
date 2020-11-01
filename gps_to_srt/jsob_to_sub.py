import datetime
import json

# ticks = 1603793423000
# ticks2 = 1603793740000

# converted_ticks = datetime.timedelta(microseconds=ticks / 10)

# converted_ticks2 = datetime.timedelta(microseconds=ticks2 / 10)

# diff = converted_ticks2 - converted_ticks
# print(diff)

# str_diff = str(diff)
# print(str_diff[0:11])


def convert_to_time(ticks):
    return datetime.timedelta(milliseconds=ticks // 10)


with open("gps.json") as f:
    data = json.load(f)

reference_time = convert_to_time(data["GPS"][0]["logTime"])

list_logtime = []

for field in data["GPS"]:

    ticks = field["logTime"]
    converted_ticks = convert_to_time(ticks)

    lat = field["lat"]
    lon = field["lon"]

    relative_time = converted_ticks - reference_time
    current_time = str(relative_time)[0:11]
    if len(current_time) < 11:
        current_time = current_time + ".000"
    current_time = current_time.replace(".", ",")
    list_logtime.append((current_time, lat, lon))

# print(list_logtime)

count = 0

with open("test.srt", "w") as f:
    for i in range(len(list_logtime) - 2):
        current_time = list_logtime[i][0]
        next_time = list_logtime[i + 1][0]

        lat = list_logtime[i][1]
        lon = list_logtime[i][2]

        if next_time != current_time:

            count += 1
            f.write(
                str(count)
                + "\n"
                + current_time
                + " --> "
                + next_time
                + "\n"
                + "lat: "
                + str(lat)
                + "\n"
                + "long: "
                + str(lon)
                + "\n\n"
            )
