use std::vec::IntoIter;
use {Error, Result};

const TOPIC_PATH_DELIMITER: char = '/';

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct TopicPath {
    pub path: String,
    // Should be false for Topic Name
    pub wildcards: bool,
    topics: Vec<Topic>
}

impl TopicPath {
    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn get(&self, index: usize) -> Option<&Topic> {
        self.topics.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Topic> {
        self.topics.get_mut(index)
    }

    pub fn len(&self) -> usize {
        self.topics.len()
    }

    pub fn is_final(&self, index: usize) -> bool {
        let len = self.topics.len();
        len == 0 || len-1 == index
    }

    pub fn is_multi(&self, index: usize) -> bool {
        match self.topics.get(index) {
            Some(topic) => *topic == Topic::MultiWildcard,
            None => false
        }
    }

    pub fn from_str<T: AsRef<str>>(path: T) -> Result<TopicPath> {
        let mut valid = true;
        let topics: Vec<Topic> = path.as_ref().split(TOPIC_PATH_DELIMITER).map( |topic| {
            match topic {
                "+" => Topic::SingleWildcard,
                "#" => Topic::MultiWildcard,
                "" => Topic::Blank,
                _ => {
                    if !Topic::validate(topic) {
                        valid = false;
                    }
                    if topic.chars().nth(0) == Some('$') {
                        Topic::System(String::from(topic))
                    } else {
                        Topic::Normal(String::from(topic))
                    }
                }
            }
        }).collect();

        if !valid {
            return Err(Error::InvalidTopicPath);
        }
        // check for wildcards
        let wildcards = topics.iter().any(|topic| {
            match *topic {
                Topic::SingleWildcard | Topic::MultiWildcard => true,
                _ => false
            }
        });

        Ok(TopicPath {
            path: String::from(path.as_ref()),
            topics: topics,
            wildcards: wildcards
        })
    }
}
