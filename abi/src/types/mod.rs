use prost_wkt_types::Timestamp;

use crate::Error;

mod document;

pub fn validate_range(start: Option<&Timestamp>, end: Option<&Timestamp>) -> Result<(), Error> {
    if start.is_none() || end.is_none() {
        return Err(Error::InvalidTime);
    }

    let start = start.unwrap();
    let end = end.unwrap();

    if start.seconds > end.seconds {
        return Err(Error::InvalidTime);
    }
    Ok(())
}
