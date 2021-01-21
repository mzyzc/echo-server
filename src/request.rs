use std::error::Error;
use base64;
use serde::Deserialize;

pub enum Operation {
    Create,
    Read,
    Update,
    Delete,
}

pub enum Target {
    Message,
    User,
}

// Canonical form of a request
pub struct Request {
    pub operation: Operation,
    pub target: Target,
    pub data: Option<Vec<u8>>,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub media_type: Option<Vec<u8>>,
    pub password: Option<String>,
    pub public_key: Option<Vec<u8>>,
    pub signature: Option<Vec<u8>>,
    pub timestamp: Option<Vec<u8>>,
}

impl Request {
    pub fn from_json(data: &str) -> Result<Request, Box<dyn Error>> {
        let request: RawRequest = serde_json::from_str(data)?;
        return request.decode()
    }
}

// Intermediate form of a request
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawRequest {
    function: String,
    data: Option<String>,
    display_name: Option<String>,
    email: Option<String>,
    media_type: Option<String>,
    password: Option<String>,
    public_key: Option<String>,
    signature: Option<String>,
    timestamp: Option<String>,
}

impl RawRequest {
    // Convert data to canonical form
    fn decode(&self) -> Result<Request, Box<dyn Error>> {
        let split_func: Vec<&str> = self.function
            .split_ascii_whitespace()
            .collect();

        return Ok(Request{
            operation: match split_func[0] {
                "CREATE" => Operation::Create,
                "READ" => Operation::Read,
                "UPDATE" => Operation::Update,
                "DELETE" => Operation::Delete,
                _ => return Err(format!("Request did not match any known operations").into()),
            },
            target: match split_func[1] {
                "MESSAGE" => Target::Message,
                "USER" => Target::User,
                _ => return Err(format!("Request did not match any known targets").into()),
            },
            data: match &self.data {
                Some(d) => Some(base64::decode(d)?),
                _ => None,
            },
            display_name: match &self.display_name {
                Some(d) => Some(String::from(d)),
                _ => None,
            },
            email: match &self.email {
                Some(d) => Some(String::from(d)),
                _ => None,
            },
            media_type: match &self.media_type {
                Some(d) => Some(base64::decode(d)?),
                _ => None,
            },
            password: match &self.password {
                Some(d) => Some(String::from(d)),
                _ => None,
            },
            public_key: match &self.public_key {
                Some(d) => Some(base64::decode(d)?),
                _ => None,
            },
            signature: match &self.signature {
                Some(d) => Some(base64::decode(d)?),
                _ => None,
            },
            timestamp: match &self.timestamp {
                Some(d) => Some(base64::decode(d)?),
                _ => None,
            },
        })
    }
}