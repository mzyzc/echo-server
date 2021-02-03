use base64;
use serde_json::Value;

pub struct User {
    pub id: Option<usize>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub password: Option<String>,
    pub public_key: Option<Vec<u8>>,
}

impl User {
    pub fn new(&self) -> Self {
        Self{
            id: self.id,

            email: if let Some(d) = &self.email {
                Some(String::from(d))
            } else { None },

            name: if let Some(d) = &self.name {
                Some(String::from(d))
            } else { None },

            public_key: if let Some(d) = &self.public_key {
                if let Ok(e) = base64::decode(d) {
                    Some(e)
                } else { None }
            } else { None },

            password: if let Some(d) = &self.password {
                Some(String::from(d))
            } else { None },
        }
    }

    pub fn from_json(data: &Value) -> Self {
        Self{
            id: None,
            email: None,
            name: None,
            password: None,
            public_key: None,
        }
    }
}

pub struct Message {
    pub id: Option<usize>,
    pub data: Option<Vec<u8>>,
    pub media_type: Option<Vec<u8>>,
    pub timestamp: Option<Vec<u8>>,
    pub signature: Option<Vec<u8>>,
}

impl Message {
    pub fn new(&self) -> Self {
        Message{
            id: self.id,

            data: if let Some(d) = &self.data {
                if let Ok(e) = base64::decode(d) {
                    Some(e)
                } else { None }
            } else { None },

            media_type: if let Some(d) = &self.media_type {
                if let Ok(e) = base64::decode(d) {
                    Some(e)
                } else { None }
            } else { None },

            timestamp: if let Some(d) = &self.timestamp {
                if let Ok(e) = base64::decode(d) {
                    Some(e)
                } else { None }
            } else { None },

            signature: if let Some(d) = &self.signature {
                if let Ok(e) = base64::decode(d) {
                    Some(e)
                } else { None }
            } else { None },
        }
    }

    pub fn from_json(data: &Value) -> Self {
        Self{
            id: None,
            data: None,
            media_type: None,
            timestamp: None,
            signature: None,
        }
    }
}

pub struct Conversation {
    pub id: Option<usize>,
    pub name: Option<String>,
}

impl Conversation {
    pub fn new(&self) -> Self {
        Conversation{
            id: self.id,

            name: if let Some(d) = &self.name {
                Some(String::from(d))
            } else { None },
        }
    }

    pub fn from_json(data: &Value) -> Self {
        Self{
            id: None,
            name: None,
        }
    }
}