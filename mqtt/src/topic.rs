use std::vec::IntoIter;
use {Error, Result};

const TOPIC_PATH_DELIMITER: char = '/';

use self::Topic::{
    Blank,
    System,
    Normal,
    SingleWildcard,
    MultiWildcard
};

impl Into<String> for Topic {
    fn into(self) -> String {
        match self {
            Normal(s) | System(s) => s,
            Blank => "".to_string(),
            SingleWildcard => "+".to_string(),
            MultiWildcard => "#".to_string()
        }
    }
}

impl Topic {
    pub fn validate(topic: &str) -> bool {
        match topic {
            "+" | "#" => true,
            _ => !(topic.contains("+") || topic.contains("#"))
        }
    }

    pub fn fit(&self, other: &Topic) -> bool {
        match *self {
	    Blank => {
                match *other {
                    Blank | SingleWildcard | MultiWildcard => true,
                    _ => false
                }
			},
	                System(ref str) => {
                match *other {
                    System(ref s) => str == s,
                    _ => false
                }
            },
            Normal(ref str) => {
                match *other {
                    Normal(ref s) => str == s,
                    SingleWildcard | MultiWildcard => true,
                    _ => false
                }
            },
            SingleWildcard => {
                match *other {
                    System(_) => false,
                    _ => true
                }
            },
            MultiWildcard => {
                match *other {
                    System(_) => false,
                    _ => true
                }
            }
        }
    }
}
