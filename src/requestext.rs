//! An extension trait for HTTP requests to work with range requests

use crate::anyrange::AnyInclusiveRange;
use ehttpd::{
    bytes::DataParseExt,
    error,
    error::Error,
    http::{Request, RequestExt},
};
use std::str;

/// An extension trait for HTTP requests to work with range requests
pub trait RequestRangeExt {
    /// The request `Range` field if any
    fn range(&self) -> Result<Option<AnyInclusiveRange<u64>>, Error>;
}
impl<'a, const HEADER_SIZE_MAX: usize> RequestRangeExt for Request<'a, HEADER_SIZE_MAX> {
    fn range(&self) -> Result<Option<AnyInclusiveRange<u64>>, Error> {
        // Get the range request
        let Some(range) = self.field("Range") else {
            return Ok(None);
        };

        // Parse the range header
        let mut range = range.clone();
        let kind = range.split_off(b"=").ok_or_else(|| error!("Invalid range header field"))?;
        if !kind.eq(b"bytes") {
            return Err(error!("Invalid range kind"))?;
        }

        // Read start and end values
        let start = range.split_off(b"-").ok_or_else(|| error!("Invalid range"))?;
        let end = range;

        // Parse the start and end values
        let start = str::from_utf8(&start)?;
        let end = str::from_utf8(&end)?;
        let range = match (start, end) {
            ("", "") => AnyInclusiveRange::Full,
            (start, "") => AnyInclusiveRange::From { start: start.parse()? },
            ("", end) => AnyInclusiveRange::To { end: end.parse()? },
            (start, end) => AnyInclusiveRange::FromTo { start: start.parse()?, end: end.parse()? },
        };
        Ok(Some(range))
    }
}
