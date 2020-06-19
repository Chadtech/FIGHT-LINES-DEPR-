use seed::browser::fetch::FetchError;

pub fn fetch_error_to_string(error: FetchError) -> String {
    match error {
        FetchError::SerdeError(error) => error.to_string(),
        FetchError::DomException(_error) => "Dom Exception".to_string(),
        FetchError::PromiseError(error) => match error.as_string() {
            Some(str) => {
                let mut buf = String::new();

                buf.push_str("Promise Error : ");
                buf.push_str(str.as_str());
                buf
            }
            None => "Unknown Promise Error".to_string(),
        },
        FetchError::NetworkError(error) => match error.as_string() {
            Some(str) => {
                let mut buf = String::new();

                buf.push_str("Network Error : ");
                buf.push_str(str.as_str());
                buf
            }
            None => "Unknown Network Error".to_string(),
        },
        FetchError::RequestError(error) => match error.as_string() {
            Some(str) => {
                let mut buf = String::new();

                buf.push_str("Request Error : ");
                buf.push_str(str.as_str());
                buf
            }
            None => "Unknown Request Error".to_string(),
        },
        FetchError::StatusError(status) => {
            let mut buf = String::new();

            buf.push_str("Code : ");
            buf.push_str(status.code.to_string().as_str());
            buf.push_str(", Message : ");
            buf.push_str(status.text.as_str());
            buf
        }
    }
}
