use std::error::Error;
use base64;
use serde_json::Value;

pub struct User {
    pub id: Option<u64>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub password: Option<String>,
    pub public_key: Option<Vec<u8>>,
}

impl User {
    pub fn from_json(data: &Value) -> Result<Self, Box<dyn Error>> {
        Ok(Self{
            id: data["id"].as_u64(),
            email: match data["email"].as_str() {
                Some(d) => Some(String::from(d)),
                None => None,
            },
            name: match data["name"].as_str() {
                Some(d) => Some(String::from(d)),
                None => None,
            },
            password: match data["password"].as_str() {
                Some(d) => Some(String::from(d)),
                None => None,
            },
            public_key: match data["publicKey"].as_str() {
                Some(d) => Some(base64::decode(d)?),
                None => None,
            },
        })
    }
}

pub struct Message {
    pub id: Option<u64>,
    pub data: Option<Vec<u8>>,
    pub media_type: Option<Vec<u8>>,
    pub timestamp: Option<Vec<u8>>,
    pub signature: Option<Vec<u8>>,
}

impl Message {
    pub fn from_json(data: &Value) -> Result<Self, Box<dyn Error>> {
        Ok(Self{
            id: data["id"].as_u64(),
            data: match data["data"].as_str() {
                Some(d) => Some(base64::decode(d)?),
                None => None,
            },
            media_type: match data["mediaType"].as_str() {
                Some(d) => Some(base64::decode(d)?),
                None => None,
            },
            timestamp: match data["timestamp"].as_str() {
                Some(d) => Some(base64::decode(d)?),
                None => None,
            },
            signature: match data["signature"].as_str() {
                Some(d) => Some(base64::decode(d)?),
                None => None,
            },
        })
    }
}

pub struct Conversation {
    pub id: Option<u64>,
    pub name: Option<Vec<u8>>,
}

impl Conversation {
    pub fn from_json(data: &Value) -> Result<Self, Box<dyn Error>> {
        Ok(Self{
            id: data["id"].as_u64(),
            name: match data["name"].as_str() {
                Some(d) => Some(base64::decode(d)?),
                None => None,
            },
        })
    }
}