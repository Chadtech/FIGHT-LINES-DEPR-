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
// CONSTS //
////////////////////////////////////////////////////////////////

static DEV_API_URL: &str = "http://localhost:2943";
static FPS_24: f64 = 41.6667;

////////////////////////////////////////////////////////////////
// INIT //
////////////////////////////////////////////////////////////////

pub fn init_dev() -> Session {
    Session {
        api_url: DEV_API_URL,
        timestamp: 0.0,
    }
}

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

    pub fn get_frame(self) -> i64 {
        (self.get_current_time() / FPS_24) as i64
    }

    pub fn asset_url(self, file_name: &'static str) -> String {
        let mut path = String::new();
        path.push_str("/assets/");
        path.push_str(file_name);
        path.push_str(".png");

        self.url(path.as_str())
    }
}

////////////////////////////////////////////////////////////////
// Tests //
////////////////////////////////////////////////////////////////

#[cfg(test)]
mod session_tests {
    use crate::session::{init_dev, FPS_24};

    #[test]
    fn within_first_frame() {
        let mut session = init_dev();

        session.set_current_time(FPS_24 - 0.001);

        assert_eq!(session.get_frame(), 0);
    }
    #[test]
    fn after_first_frame() {
        let mut session = init_dev();

        session.set_current_time(FPS_24 + 0.001);

        assert_eq!(session.get_frame(), 1);
    }
}
