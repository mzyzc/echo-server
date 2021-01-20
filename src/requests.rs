use std::error::Error;
use std::str;
use base64;
use serde::{Serialize, Deserialize};

enum Operation {
    Create,
    Read,
    Update,
    Delete,
}

enum Target {
    Message,
    User,
}

struct Request {
    operation: Operation,
    target: Target,
    data: Vec<u8>,
    display_name: String,
    email: String,
    media_type: Vec<u8>,
    password: String,
    public_key: Vec<u8>,
    signature: Vec<u8>,
    timestamp: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
struct RawRequest {
    function: String,
    data: String,
    displayName: String,
    email: String,
    mediaType: String,
    password: String,
    publicKey: String,
    signature: String,
    timestamp: String,
}

impl RawRequest {
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
            data: base64::decode(&self.data)?,
            display_name: String::from(&self.displayName),
            email: String::from(&self.email),
            media_type: base64::decode(&self.mediaType)?,
            password: String::from(&self.password),
            public_key: base64::decode(&self.publicKey)?,
            signature: base64::decode(&self.signature)?,
            timestamp: base64::decode(&self.timestamp)?,
        })
    }
}

pub fn parse_request(data: &[u8]) -> Result<(), Box<dyn Error>> {
    let data = str::from_utf8(data)?;
    println!("{}", data);
    let request: RawRequest = serde_json::from_str(data)?;
    let _request: Request = request.decode()?;

    Ok(())
}