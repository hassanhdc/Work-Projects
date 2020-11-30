use bytes::BytesMut;
use mqttrs::{decode, encode, Error, Packet};
use tokio_util::codec::{Decoder, Encoder};

pub struct MQTTCodec {}

impl MQTTCodec {
    pub fn new() -> MQTTCodec {
        MQTTCodec {}
    }
}

impl Encoder<Packet> for MQTTCodec {
    type Error = Error;

    fn encode(&mut self, packet: Packet, mut buf: &mut BytesMut) -> Result<(), Self::Error> {
        encode(&packet, &mut buf)
    }
}

impl Decoder for MQTTCodec {
    type Item = Packet;
    type Error = Error;
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        decode(src)
    }
}
