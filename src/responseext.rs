//! An extension trait for HTTP requests to work with range requests

use crate::rangeext::RangeExt;
use ehttpd::{
    bytes::{Data, DataSliceExt, Source},
    error,
    error::Error,
    http::{Response, ResponseExt},
};
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    ops::{Range, RangeBounds, RangeInclusive},
};

/// An extension trait for HTTP responses to work with range requests
pub trait ResponseRangeExt
where
    Self: Sized,
{
    /// Creates a new `206 Partial Content` HTTP response
    fn new_206_partial_content() -> Self;

    /// Sets the `Accept-Ranges` to `bytes`
    fn set_accept_ranges_bytes(&mut self);
    /// Sets the `Accept-Ranges` to `none`
    fn set_accept_ranges_none(&mut self);

    /// Sets the `Content-Range` header
    fn set_content_range<T>(&mut self, range: T, total: u64) -> Result<(), Error>
    where
        T: RangeBounds<u64>;

    /// Sets the body for a `Partial Range` response
    ///
    /// # Note
    /// This function also sets the `Content-Length` and the `Content-Range` headers. Furthermore, it raises an error if
    /// `self.status` is not `206`
    fn set_body_data_range<T, R>(&mut self, data: T, range: R) -> Result<(), Error>
    where
        T: Into<Data>,
        R: RangeBounds<usize>;
    /// Sets the body for a `Partial Range` response
    ///
    /// # Note
    /// This function also sets the `Content-Length` and the `Content-Range` headers. Furthermore, it raises an error if
    /// `self.status` is not `206`
    fn set_body_file_range<T, R>(&mut self, file: T, range: R) -> Result<(), Error>
    where
        T: Into<File>,
        R: RangeBounds<u64>;
}
impl<const HEADER_SIZE_MAX: usize> ResponseRangeExt for Response<HEADER_SIZE_MAX> {
    fn new_206_partial_content() -> Self {
        Self::new_status_reason(206, "Partial Content")
    }

    fn set_accept_ranges_bytes(&mut self) {
        self.set_field("Accept-Ranges", "bytes")
    }
    fn set_accept_ranges_none(&mut self) {
        self.set_field("Accept-Ranges", "none")
    }

    fn set_content_range<T>(&mut self, range: T, total: u64) -> Result<(), Error>
    where
        T: RangeBounds<u64>,
    {
        // Compute the bounds
        let range = RangeInclusive::from_range_bounds(range, 0, total)
            .ok_or_else(|| error!("Range would exceed total limit"))?;
        let range_string = format!("{}-{}/{total}", range.start(), range.end());

        // Set the range
        self.set_field("Content-Range", range_string);
        Ok(())
    }

    fn set_body_data_range<T, R>(&mut self, data: T, range: R) -> Result<(), Error>
    where
        T: Into<Data>,
        R: RangeBounds<usize>,
    {
        // Ensure that we are a 206
        if !self.status.eq(b"206") {
            return Err(error!("Response is not a 206 response"));
        }

        // Prepare data and range
        let data: Data = data.into();
        let Range { start, end } =
            Range::from_range_bounds(range, 0, data.len()).ok_or_else(|| error!("Range would exceed data size"))?;
        let subdata = data.subcopy(start..end).expect("range would exceed data size");

        // Set content-range header and body data
        self.set_content_range(start as u64..end as u64, data.len() as u64)?;
        self.set_body_data(subdata);
        Ok(())
    }
    fn set_body_file_range<T, R>(&mut self, file: T, range: R) -> Result<(), Error>
    where
        T: Into<File>,
        R: RangeBounds<u64>,
    {
        // Ensure that we are a 206
        if !self.status.eq(b"206") {
            return Err(error!("Response is not a 206 response"));
        }

        // Open the file and get the file size
        let mut file = file.into();
        let file_size = file.metadata()?.len();
        let Range { start, end } =
            Range::from_range_bounds(range, 0, file_size).ok_or_else(|| error!("Range would exceed file size"))?;

        // Get the length and virtually truncate the file
        let len = end.saturating_sub(start);
        file.seek(SeekFrom::Start(start))?;
        let file = file.take(end.saturating_sub(start));

        // Set content-range and content-length header
        self.set_content_range(start..end, file_size)?;
        self.set_content_length(len);

        // Set the raw body
        let source = Source::from_other(file);
        self.set_body(source);
        Ok(())
    }
}
