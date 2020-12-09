from socket import timeout
import paho.mqtt.client as mqtt_client
import time

host = "10.227.141.216"
# host = "127.0.0.1"
port = 1883
# topic_incoming = "mqtt/incoming"
# topic_outgoing = "mqtt/outgoing"
topic = "bw3/presence"
topic_ping = "bw3/4/WKBXXB0054/ping"
topic_pong = "bw3/4/WKBXXB0054/pong"
topic_outgoing = "bw3/4/WKBXXB0054/outgoing"

client_id = "WKBXXB0054"


def connect_mqtt():
    def on_connect(client, userdata, flags, rc):
        if rc == 0:
            print("Connected to MQTT Broker!")
        else:
            print("Failed to connect, return code %d\n", rc)

    client = mqtt_client.Client(client_id)
    client.on_connect = on_connect
    lwm = """{"ClientId":"WKBXXB0054","StationId":4,"Status":0}"""
    client.will_set(topic, lwm, qos=1, retain=False)
    client.connect(host, port, keepalive=120)
    return client


def subscribe(client: mqtt_client):
    def on_message(client, userdata, msg):
        print(f"Received '{msg.payload.decode()}' from {msg.topic}")
        time.sleep(2)
        publish_pong(client, msg.payload)
        # time.sleep(2)

    client.subscribe(topic_ping)
    client.subscribe(topic_outgoing)
    client.on_message = on_message


def publish(client):
    msg = """{"ClientId":"WKBXXB0054","StationId":4,"Status":1}"""
    result = client.publish(topic, msg)
    status = result[0]
    if status == 0:
        print(f"Send presence data to topic `{topic_pong}`")
    else:
        print(f"Failed to send message to topic {topic_pong}")
    # time.sleep(240)


def publish_pong(client, msg):
    result = client.publish(topic_pong, msg)
    status = result[0]
    if status == 0:
        print(f"Send pong data to topic `{topic_pong}`")
    else:
        print(f"Failed to send message to topic {topic_pong}")


def run():
    client = connect_mqtt()
    publish(client)
    subscribe(client)
    client.loop_forever(timeout=240)


if __name__ == '__main__':
    run()
