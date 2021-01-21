use crate::request;

use std::error::Error;
use std::str;

pub fn parse_request(data: &[u8]) -> Result<(), Box<dyn Error>> {
    // Prepare data
    let data = str::from_utf8(data)?
        .trim_matches('\0');
    let _request = request::Request::from_json(data)?;

    Ok(())
}