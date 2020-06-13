////////////////////////////////////////////////////////////////
// TYPES //
////////////////////////////////////////////////////////////////

#[derive(Copy, Clone)]
pub struct Session {
    api_url: &'static str,

    /// Not the time stamp since 1970
    /// rather, the time stamp since the
    /// beginning of browser session
    timestamp: f64,
}

////////////////////////////////////////////////////////////////
// INIT //
////////////////////////////////////////////////////////////////

pub fn init_dev() -> Session {
    Session {
        api_url: DEV_API_URL,
        timestamp: 0.0,
    }
}

static DEV_API_URL: &str = "http://localhost:2943";

////////////////////////////////////////////////////////////////
// API //
////////////////////////////////////////////////////////////////

impl Session {
    pub fn url(self, path: &str) -> String {
        let mut buf: String = String::new();

        buf.push_str(self.api_url);
        buf.push_str(path);

        buf
    }

    pub fn set_current_time(&mut self, timestamp: f64) {
        self.timestamp = timestamp;
    }

    pub fn get_current_time(self) -> f64 {
        self.timestamp
    }
}
