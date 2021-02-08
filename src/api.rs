pub mod request;
pub mod response;

use std::convert::TryFrom;
use std::error::Error;
use base64;
use serde_json::Value;
#[derive(Clone)]
pub struct User {
    pub id: Option<i32>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub password: Option<String>,
    pub public_key: Option<Vec<u8>>,
}

impl User {
    // Create a user object from JSON
    pub fn from_json(data: &Value) -> Result<Self, Box<dyn Error>> {
        Ok(Self{
            id: match data["id"].as_i64() {
                Some(d) => Some(i32::try_from(d)?),
                None => None,
            },
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
    pub id: Option<i32>,
    pub data: Option<Vec<u8>>,
    pub media_type: Option<Vec<u8>>,
    pub timestamp: Option<Vec<u8>>,
    pub signature: Option<Vec<u8>>,
    pub sender: Option<String>,
}

impl Message {
    // Create a message object from JSON
    pub fn from_json(data: &Value) -> Result<Self, Box<dyn Error>> {
        Ok(Self{
            id: match data["id"].as_i64() {
                Some(d) => Some(i32::try_from(d)?),
                None => None,
            },
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
            sender: match data["sender"].as_str() {
                Some(d) => Some(String::from(d)),
                None => None,
            }
        })
    }
}

#[derive(Clone)]
pub struct Conversation {
    pub id: Option<i32>,
    pub name: Option<Vec<u8>>,
}

impl Conversation {
    // Create a conversation object from JSON
    pub fn from_json(data: &Value) -> Result<Self, Box<dyn Error>> {
        Ok(Self{
            id: match data["id"].as_i64() {
                Some(d) => Some(i32::try_from(d)?),
                None => None,
            },
            name: match data["name"].as_str() {
                Some(d) => Some(base64::decode(d)?),
                None => None,
            },
        })
    }
}