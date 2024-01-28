//! Implements a common range-request-response pattern and fuzzes it

use ehttpd::{
    bytes::{Data, Source},
    http::{Request, Response},
};
use ehttpd_range::{RequestRangeExt, ResponseRangeExt};
use rand::{rngs::ThreadRng, Rng};
use std::{ops::Deref, sync::Arc, thread};

/// Some fake data
#[derive(Debug, Clone)]
struct FakeData {
    /// Some fake data
    data: Arc<Vec<u8>>,
}
impl FakeData {
    /// Generates some random fake data
    pub fn generate(max: usize, rng: &mut ThreadRng) -> Self {
        let mut data = vec![0u8; max];
        rng.fill(data.as_mut_slice());
        Self { data: Arc::new(data) }
    }
}
impl Deref for FakeData {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl From<FakeData> for Data {
    fn from(value: FakeData) -> Self {
        Data::ArcVec { backing: Arc::clone(&value.data), range: 0..value.data.len() }
    }
}

/// A fake request
#[derive(Debug, Clone)]
struct FakeRequest {
    /// Some fake data
    data: FakeData,
    /// The start within the fake data
    start: usize,
    /// The end within the fake data (inclusive)
    end_incl: usize,
}
impl FakeRequest {
    /// Creates a random fake request over the fake data
    pub fn generate(data: &FakeData, rng: &mut ThreadRng) -> Self {
        let len_incl = data.len().saturating_sub(1);
        let start = rng.gen_range(0..=len_incl);
        let end_incl = rng.gen_range(start..=len_incl);
        Self { data: data.clone(), start, end_incl }
    }

    /// Performs a roundtrip
    pub fn roundtrip(self) {
        // Build the request
        let request_data =
            format!(concat!("GET / HTTP/1.1\r\n", "Range: bytes={}-{}\r\n", "\r\n"), self.start, self.end_incl);
        let mut request_stream = Source::from(request_data);
        let request: Request = Request::from_stream(&mut request_stream)
            .expect("failed to parse request")
            .expect("unexpected empty request");

        // "Handle" the GET request
        assert_eq!(request.method.as_ref(), b"GET");
        assert_eq!(request.target.as_ref(), b"/");
        assert_eq!(request.version.as_ref(), b"HTTP/1.1");

        // Validate range
        let range = (request.range())
            .expect("invalid HTTP range request")
            .expect("missing expected range header")
            .to_inclusive(0, self.data.len() as u64)
            .expect("invalid range in range header");
        assert_eq!(range, (self.start as u64)..=(self.end_incl as u64));

        // Create the response data
        let response_data = {
            // Create the response
            let mut response: Response = ResponseRangeExt::new_206_partial_content();
            response.set_accept_ranges_bytes();
            response
                .set_body_data_range(self.data.clone(), self.start..=self.end_incl)
                .expect("failed to set range body");

            // Serialize the response
            let mut response_data = Vec::new();
            response.to_stream(&mut response_data).expect("failed to write range body");
            response_data
        };

        // Validate the response
        let expected = format!(
            concat!(
                "HTTP/1.1 206 Partial Content\r\n",
                "Accept-Ranges: bytes\r\n",
                "Content-Range: bytes {}-{}/{}\r\n",
                "Content-Length: {}\r\n",
                "\r\n"
            ),
            self.start,
            self.end_incl,
            self.data.len(),
            self.end_incl.saturating_sub(self.start) + 1
        );
        let mut expected = expected.into_bytes();
        expected.extend_from_slice(&self.data[self.start..=self.end_incl]);
        assert_eq!(expected, response_data);
    }
}

/// A thread's main function
fn thread_main() {
    /// The maximum fake data size
    const MAX: usize = 128 * 1024 * 1024;

    // Generate fake data
    let mut rng = rand::thread_rng();
    let data = FakeData::generate(MAX, &mut rng);
    loop {
        // Create a fake request
        let fake = FakeRequest::generate(&data, &mut rng);
        fake.roundtrip();
    }
}

fn main() {
    // Get the amount of available parallelism
    let threadnum = match thread::available_parallelism() {
        Ok(threadnum) => usize::from(threadnum),
        Err(_) => 1,
    };

    // Spawn the threads
    let mut threads = Vec::with_capacity(threadnum);
    for _ in 0..threadnum {
        let thread = thread::spawn(thread_main);
        threads.push(thread);
    }

    // Await until the threads are done
    for thread in threads {
        thread.join().expect("thread has panicked");
    }
}
