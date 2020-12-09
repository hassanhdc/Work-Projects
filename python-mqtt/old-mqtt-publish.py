import paho.mqtt.client as mqtt_client
import time
import random

host = "10.227.141.1"
port = 1883
topic_incoming = "mqtt/incoming"
topic_outgoing = "mqtt/outgoing"
topic = "bw3/presence"

client_id = f"python-mqtt-{random.randint(0,100)}"


def connect_mqtt() -> mqtt_client:
    def on_connect(client, userdata, flags, rc):
        if rc == 0:
            print("Connected to MQTT Broker!")
        else:
            print("Failed to connect, return code ", rc)

    client = mqtt_client.Client(client_id)
    client.on_connect = on_connect
    client.connect(host, port)
    return client


def subscribe(client: mqtt_client):
    def on_message(client, userdata, msg):
        print(f"Received '{msg.payload.decode()}' from {msg.topic}")

    client.subscribe(topic)
    client.on_message = on_message


def publish(client: mqtt_client):
    msg_count = 0
    while True:
        time.sleep(1)
        msg = f"messages: {msg_count}"
        result = client.publish(topic_incoming, msg)
        status = result[0]
        if status == 0:
            print(f"Sent {msg} to topic {topic_incoming}")
        else:
            print(f"Failed to send message to topic {topic_incoming}")

        msg_count += 1


def run():
    client = connect_mqtt()
    client.loop_start()
    publish(client)


run()
