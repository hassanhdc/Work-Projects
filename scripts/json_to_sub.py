import datetime
import json


def ticks_to_time(ticks):
    """
    Returns converted .NET Ticks to
    time in "HH : MM : SS : MS" format
    """

    return datetime.datetime(1970, 1, 1) + datetime.timedelta(hours=5, seconds=ticks // 1000)


with open("gps.json") as f:
    data = json.load(f)


record_logtime = []
start_time = ticks_to_time(
    data["GPS"][0]["logTime"])

for field in data["GPS"]:

    ticks = field["logTime"]
    lat = field["lat"]
    lon = field["lon"]
    speed = field["speed"]

    converted_ticks = ticks_to_time(ticks)
    relative_time = converted_ticks - start_time

    time = str(converted_ticks)

    # Pad time string with '000' millis
    current_time = str(relative_time) + ",000"

    # if len(current_time) < 11:
    #     current_time = current_time + ".000"

    # current_time = current_time.replace(".", ",")

    record_logtime.append((current_time, lat, lon, speed, time))


with open("subtitles.srt", "w") as f:
    count = 0

    for i in range(len(record_logtime) - 2):
        current_time = record_logtime[i][0]
        next_time = record_logtime[i + 1][0]

        lat = record_logtime[i][1]
        lon = record_logtime[i][2]
        speed = record_logtime[i][3]
        time = record_logtime[i][4]

        if next_time != current_time:

            count += 1
            f.write(
                str(count)
                + "\n"
                + current_time
                + " --> "
                + next_time
                + "\n"
                + time
                + 24 * " \n"
                + "lat: "
                + str(lat)
                + 159 * "\t"
                + "speed: "
                + str(speed)
                + "\n"
                + "lon: "
                + str(lon)
                + "\n\n"
            )
