use chrono::{DateTime, Utc};
use serde_derive::*;

/// JSONの{"user_agent": "xxx", "response_time": 0, "timestamp": "yyyy-MM-dd+HH:mm:ss"}に対応
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Log {
    pub user_agent: String,
    pub response_time: i32,
    pub timestamp: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
