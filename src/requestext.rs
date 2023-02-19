//! An extension trait for HTTP requests to work with range requests

use ehttpd::{
    bytes::DataParseExt,
    error,
    error::Error,
    http::{Request, RequestExt},
};
use std::{ops::RangeInclusive, str};

/// An extension trait for HTTP requests to work with range requests
pub trait RequestRangeExt {
    /// The request `Range` field if any
    fn range(&self) -> Result<Option<RangeInclusive<u64>>, Error>;
}
impl<'a, const HEADER_SIZE_MAX: usize> RequestRangeExt for Request<'a, HEADER_SIZE_MAX> {
    fn range(&self) -> Result<Option<RangeInclusive<u64>>, Error> {
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

        // Split the range
        let start = range.split_off(b"-").ok_or_else(|| error!("Invalid range"))?;
        let end_incl = range;

        // Parse the start and end values
        let start: u64 = {
            let start_str = str::from_utf8(&start)?;
            start_str.parse()?
        };
        let end_incl: u64 = {
            let end_incl_str = str::from_utf8(&end_incl)?;
            end_incl_str.parse()?
        };
        Ok(Some(start..=end_incl))
    }
}
