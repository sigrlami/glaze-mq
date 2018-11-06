use std::sync::Arc;
use std::vec::Vec;
use {Publish, TopicPath, PacketIdentifier, QoS, LastWill, Error, Result};

#[derive(Debug, Clone)]
pub struct Message {
    pub topic: TopicPath,
    pub qos: QoS,
    pub retain: bool,

    pub pid: Option<PacketIdentifier>,
    pub payload: Arc<Vec<u8>>
}

impl Message {
    pub fn from_pub(publish: Box<Publish>) -> Result<Box<Message>> {
        let topic = TopicPath::from(publish.topic_name.as_str());
        if topic.wildcards {
            return Err(Error::TopicNameMustNotContainWildcard);
        }
        Ok(Box::new(Message {
            topic: topic,
            qos: publish.qos,
            retain: publish.retain,
            pid: publish.pid,
            payload: publish.payload.clone()
        }))
    }

    pub fn from_last_will(last_will: LastWill) -> Box<Message> {
        let topic = TopicPath::from(last_will.topic);

        Box::new(Message {
            topic: topic,
            qos: last_will.qos,
            retain: last_will.retain,
            pid: None,
            payload: Arc::new(last_will.message.into_bytes())
        })
    }

    pub fn to_pub(&self, qos: Option<QoS>, dup: bool) -> Box<Publish> {
        let qos = qos.unwrap_or(self.qos);
        Box::new(Publish {
            dup: dup,
            qos: qos,
            retain: self.retain,
            topic_name: self.topic.path.clone(),
            pid: self.pid,
            payload: self.payload.clone()
        })
    }

    pub fn transform(&self, pid: Option<PacketIdentifier>, qos: Option<QoS>) -> Box<Message> {
        let pid = pid.or(self.pid);
        let qos = qos.unwrap_or(self.qos);
        Box::new(Message {
            topic: self.topic.clone(),
            qos: qos,
            retain: self.retain,
            pid: pid,
            payload: self.payload.clone()
        })
    }
}
