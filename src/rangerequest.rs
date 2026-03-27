//! An extension trait for HTTP requests to work with range requests

use crate::anyrange::AnyInclusiveRange;
use ehttpd::bytes::Parse;
use ehttpd::err;
use ehttpd::error::Error;
use ehttpd::http::Request;

/// An extension trait for HTTP requests to work with range requests
pub trait RangeRequest {
    /// The request `Range` field if any
    fn range(&self) -> Result<Option<AnyInclusiveRange<u64>>, Error>;
}
impl<'a, const HEADER_SIZE_MAX: usize> RangeRequest for Request<'a, HEADER_SIZE_MAX> {
    fn range(&self) -> Result<Option<AnyInclusiveRange<u64>>, Error> {
        // Get the range request
        let Some(range) = self.field("Range") else {
            return Ok(None);
        };

        // Parse the range header
        let range = &mut range.as_ref();
        let kind = Parse::split_off(range, b"=").ok_or_else(|| err!("Invalid range header field"))?;
        if !kind.eq(b"bytes") {
            return Err(err!("Invalid range kind"))?;
        }

        // Read start and end values
        let start = Parse::split_off(range, b"-").ok_or_else(|| err!("Invalid range"))?;
        let end = range;

        // Parse the start and end values
        let range = match (start.as_ref(), end.as_ref()) {
            (b"", b"") => AnyInclusiveRange::Full,
            (start, b"") => AnyInclusiveRange::From { start: start.parse()? },
            (b"", end) => AnyInclusiveRange::To { end: end.parse()? },
            (start, end) => AnyInclusiveRange::FromTo { start: start.parse()?, end: end.parse()? },
        };
        Ok(Some(range))
    }
}
