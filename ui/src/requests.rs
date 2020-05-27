use seed::fetch;
use std::fmt::Debug;

static BASE_API_URL: &str = "https://localhost:2943";
const TIMEOUT: u32 = 5000;

pub fn new(path: &str) -> fetch::Request {
    fetch::Request::new(format!("{}/{}", BASE_API_URL, path)).timeout(TIMEOUT)
}
