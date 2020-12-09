import paho.mqtt.client as mqtt_client
import time

host = "10.227.141.112"
# host = "127.0.0.1"
port = 1883

topic_presence = "bw3/presence"
topic_ping = "bw3/4/WKBXXB0054/ping"
topic_pong = "bw3/4/WKBXXB0054/pong"
topic_outgoing = "bw3/4/WKBXXB0054/outgoing"
topic_incoming = "bw3/4/WKBXXB0054/incoming"

topics_sub = [topic_ping, topic_outgoing]

client_id = "WKBXXB0054"
lwm = """{"ClientId":"WKBXXB0054","StationId":4,"Status":0}"""
presence_online = """{"ClientId":"WKBXXB0054","StationId":4,"Status":1}"""
presence_offline = """{"ClientId":"WKBXXB0054","StationId":4,"Status":0}"""


def on_message(client, userdata, msg):
    if msg.topic == topic_ping:
        print(
            f"Received message on topic {msg.topic} = {msg.payload.decode()}")
        client.publish(topic_pong, msg.payload)

    if msg.topic == topic_outgoing:
        print(
            f"Received message on topic {msg.topic} = {msg.payload.decode()}")
        time.sleep(1)
        client.publish(topic_incoming, msg.payload)
    else:
        pass


def on_connect(client, userdata, flags, rc):
    if rc == 0:
        print("Connected to MQTT Broker")
        time.sleep(1)
        subscribe_topics(client)
    else:
        exit()


def on_disconnect(client, userdata, rc):
    client.publish(topic_presence, presence_offline)


def ping_msg_cb(client, userdata, msg):
    print("Received message from {msg.topic} : {msg.payload.decode()}")
    print("Replying to message topic from {topic_pong} - pong")

    client.publish(topic_pong, msg.payload)


def outgoing_msg_cb(client, userdata, msg):
    print("Received message from {msg.topic} : {msg.payload.decode()}")
    print("Replying to message from {topic_incoming}")

    client.publish(topic_incoming, msg.payload)


def subscribe_topics(client):
    for topic in topics_sub:
        client.subscribe(topic)
        print("Subscribed to topic {topic}")

    client.message_callback_add(topic_ping, ping_msg_cb)
    client.message_callback_add(topic_outgoing, outgoing_msg_cb)


client = mqtt_client.Client(client_id)
client.on_connect = on_connect
client.on_message = on_message

client.will_set(topic_presence, lwm, qos=1, retain=False)

client.connect(host, port)

client.loop_start()
client.publish(topic_presence, presence_online)

time.sleep(10)

client.disconnect(reasoncode=0)
client.loop_stop()
