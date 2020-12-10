import paho.mqtt.client as mqtt_client
import time

host = "10.227.141.112"
# host = "127.0.0.1"
port = 1883

topic_presence = "bw3/presence"
topic_ping = "bw3/4/WKBXXB0054/ping"
topic_pong = "bw3/4/WKBXXB0054/pong"
topic_evm = "bw3/4/WKBXXB0054/evm3"
topic_iot = "bw3/4/WKBXXB0054/iot"

topics_sub = [topic_ping, topic_evm]

client_id = "WKBXXB0054"
lwm = """{"ClientId":"WKBXXB0054","StationId":4,"Status":0}"""
presence_online = """{"ClientId":"WKBXXB0054","StationId":4,"Status":1}"""
presence_offline = """{"ClientId":"WKBXXB0054","StationId":4,"Status":0}"""
# ? 1st attempt
# incoming_msg = """{"ConversationID":"23439f1c-98f1-4bbd-8e22-e3f86055e6df","MessageType":"Ack","MessageOrigin":null,"From":"WKBXXB0054","CommandInvokerGuid":null,"To":"admin","TimestampOrigin":1607499037857,"TimestampHub":1607499037857,"Body":"{\r\n  \"Code\": 200,\r\n  \"Description\": \"Message Received\",\r\n  \"OriginalMessage\": {\r\n    \"ConversationID\": \"23439f1c-98f1-4bbd-8e22-e3f86055e6df\",\r\n    \"MessageType\": \"Command\",\r\n    \"MessageOrigin\": null,\r\n    \"From\": \"admin\",\r\n    \"CommandInvokerGuid\": \"012bf825-73b5-4eea-afa5-94e41ae157de\",\r\n    \"To\": \"WKBXXB0054\",\r\n    \"TimestampOrigin\": 1607499037775,\r\n    \"TimestampHub\": 1607499037776,\r\n    \"Body\": \"{\\\"CommandType\\\":\\\"InCarStatus\\\",\\\"CommandID\\\":10501,\\\"Params\\\":{\\\"Duration\\\":60}}\",\r\n    \"Data\": null\r\n  }\r\n}","Data":null}"""
# #                  """{"ConversationID":"23439f1c-98f1-4bbd-8e22-e3f86055e6df","MessageType":"Success","MessageOrigin":null,"From":"WKBXXB0054","CommandInvokerGuid":"012bf825-73b5-4eea-afa5-94e41ae157de","To":"admin","TimestampOrigin":1607499037775,"TimestampHub":1607499037776,"Body":"{\"success\": true,\"Result\": \"{\\\"success\\\":true,\\\"Data\\\":{\\\"status\\\":\\\"StreamingOff\\\",\\\"commandInvoker\\\":\\\"\\\"}}\",\"CommandType\": \"InCarStatus\",\"CommandID\": 10501,\"Description\": \"Operation succeeded.\"}","Data":null}"""
# #  ]

# ? 2nd attempt
iot_reply_1 = """{\"ConversationID\":\"13c618d8-c20f-4fc2-a93c-af2fead8ac6a\",\"MessageType\":\"Ack\",\"MessageOrigin\":null,\"From\":\"WKBXXB0054\",\"CommandInvokerGuid\":null,\"To\":\"admin\",\"TimestampOrigin\":1607599458679,\"TimestampHub\":1607599458679,\"Body\":\"{\\r\\n  \\\"Code\\\": 200,\\r\\n  \\\"Description\\\": \\\"Message Received\\\",\\r\\n  \\\"OriginalMessage\\\": {\\r\\n    \\\"ConversationID\\\": \\\"13c618d8-c20f-4fc2-a93c-af2fead8ac6a\\\",\\r\\n    \\\"MessageType\\\": \\\"Command\\\",\\r\\n    \\\"MessageOrigin\\\": null,\\r\\n    \\\"From\\\": \\\"admin\\\",\\r\\n    \\\"CommandInvokerGuid\\\": \\\"0841ce8b-f263-435c-b455-f66fd865070e\\\",\\r\\n    \\\"To\\\": \\\"WKBXXB0054\\\",\\r\\n    \\\"TimestampOrigin\\\": 1607599458675,\\r\\n    \\\"TimestampHub\\\": 1607599458675,\\r\\n    \\\"Body\\\": \\\"{\\\\\\\"CommandType\\\\\\\":\\\\\\\"InCarStatus\\\\\\\",\\\\\\\"CommandID\\\\\\\":10501,\\\\\\\"Params\\\\\\\":{\\\\\\\"Duration\\\\\\\":60}}\\\",\\r\\n    \\\"Data\\\": null\\r\\n  }\\r\\n}\",\"Data\":null
}"""
iot_reply_2 = """{\"ConversationID\":\"f5dd122b-83b0-4a0f-b18a-eb8bafa8ccb5\",\"MessageType\":\"Success\",\"MessageOrigin\":null,\"From\":\"WKBXXB0054\",\"CommandInvokerGuid\":\"7349a511-51e5-4b82-bf36-8900192b788f\",\"To\":\"admin\",\"TimestampOrigin\":1607601281779,\"TimestampHub\":1607601281780,\"Body\":\"{\\\"success\\\": true,\\\"Result\\\": \\\"{\\\\\\\"success\\\\\\\":true,\\\\\\\"Data\\\\\\\":{\\\\\\\"status\\\\\\\":\\\\\\\"StreamingOff\\\\\\\",\\\\\\\"commandInvoker\\\\\\\":\\\\\\\"\\\\\\\"}}\\\",\\\"CommandType\\\": \\\"InCarStatus\\\",\\\"CommandID\\\": 10501,\\\"Description\\\": \\\"Operation succeeded.\\\"}\",\"Data\":null}"""


def on_message(client, userdata, msg):
    print(f"Received message on topic {msg.topic} = {msg.payload.decode()}")


def on_connect(client, userdata, flags, rc):
    if rc == 0:
        print("Connected to MQTT Broker")
        time.sleep(1)
        subscribe_topics(client)
        client.publish(topic_presence, presence_online)
    else:
        exit()


def disconnect_from_evm(client):
    client.publish(topic_presence, lwm)
    client.disconnect()


def ping_msg_cb(client, userdata, msg):
    print(f"\nReceived PING on {msg.topic} : {msg.payload.decode()}")
    print(f"Replying PONG on {topic_pong}")

    client.publish(topic_pong, msg.payload)


def outgoing_msg_cb(client, userdata, msg):
    print(f"\nReceived COMMAND on {msg.topic} : {msg.payload.decode()}")
    print(f"Acknowledging on {topic_iot}")

    client.publish(topic_iot, iot_reply_1)
    client.publish(topic_iot, iot_reply_2)
    # client.publish(topic_iot, incoming_msgs[1])


def subscribe_topics(client):
    for topic in topics_sub:
        client.subscribe(topic)
        print(f"Subscribed to topic {topic}")

    client.message_callback_add(topic_ping, ping_msg_cb)
    client.message_callback_add(topic_evm, outgoing_msg_cb)


client = mqtt_client.Client(client_id)
client.will_set(topic_presence, lwm, qos=1, retain=False)

client.on_connect = on_connect
client.on_message = on_message

client.connect(host, port)

client.loop_start()

time.sleep(200)

client.disconnect_from_evm()
client.loop_stop()
