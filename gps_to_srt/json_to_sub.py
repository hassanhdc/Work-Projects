import datetime
import json


def ticks_to_time(ticks):
    """
    Returns converted .NET Ticks to
    time in "HH : MM : SS : MS" format
    """

    return datetime.timedelta(seconds=ticks // 1000)


with open("gps.json") as f:
    data = json.load(f)


record_logtime = []
start_time = ticks_to_time(data["GPS"][0]["logTime"])

for field in data["GPS"]:

    ticks = field["logTime"]
    lat = field["lat"]
    lon = field["lon"]
    speed = field["speed"]

    converted_ticks = ticks_to_time(ticks)

    relative_time = converted_ticks - start_time

    # Chop off extra-milliseconds - measure in 100ms
    current_time = str(relative_time)[0:11]

    # Pad time string with '000 to have uniform length
    if len(current_time) < 11:
        current_time = current_time + ".000"

    current_time = current_time.replace(".", ",")

    record_logtime.append((current_time, lat, lon, speed))


with open("subtitles.srt", "w") as f:
    count = 0

    for i in range(len(record_logtime) - 2):
        current_time = record_logtime[i][0]
        next_time = record_logtime[i + 1][0]

        lat = record_logtime[i][1]
        lon = record_logtime[i][2]
        speed = record_logtime[i][3]

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
                + 110 * "\t"
                + "long: "
                + str(lon)
                + "\n"
                + "speed: "
                + str(speed)
                + "\n\n"
            )
