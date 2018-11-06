#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    MQIsdp(u8),
    MQTT(u8)
}

impl Protocol {
    pub fn new(name: &str, level: u8) -> Result<Protocol> {
        match name {
            "MQIsdp" => match level {
                3 => Ok(Protocol::MQIsdp(3)),
                _ => Err(Error::UnsupportedProtocolVersion)
            },
            "MQTT" => match level {
                4 => Ok(Protocol::MQTT(4)),
                _ => Err(Error::UnsupportedProtocolVersion)
            },
            _ => Err(Error::UnsupportedProtocolName)
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            &Protocol::MQIsdp(_) => "MQIsdp",
            &Protocol::MQTT(_) => "MQTT"
        }
    }

    pub fn level(&self) -> u8 {
        match self {
            &Protocol::MQIsdp(level) => level,
            &Protocol::MQTT(level) => level
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QoS {
    AtMostOnce,
    AtLeastOnce,
    ExactlyOnce
}
