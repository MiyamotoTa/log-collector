use chrono::{DateTime, Utc};
use serde_derive::*;

/// JSONの{"user_agent": "xxx", "response_time": 0, "timestamp": "yyyy-MM-dd+HH:mm:ss"}に対応
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Log {
    pub user_agent: String,
    pub response_time: i32,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct DateTimeRange {
    pub from: Option<DateTime<Utc>>,
    pub until: Option<DateTime<Utc>>,
}

pub mod csv {
    pub mod get {
        use crate::DateTimeRange;
        pub type Query = DateTimeRange;
        // GETはファイルを返すのでResponse型の定義がない
    }

    pub mod post {
        use serde_derive::*;

        // CSVファイルを受け付けるのでリクエストデータはない
        #[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
        // 受領したログの数を返す
        pub struct Response(pub usize);
    }
}

pub mod logs {
    pub mod get {
        use crate::{DateTimeRange, Log};
        use serde_derive::*;

        pub type Query = DateTimeRange;

        #[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
        // 保存しているログをすべて返す
        pub struct Response(pub Vec<Log>);
    }

    pub mod post {
        use chrono::{DateTime, Utc};
        use serde_derive::*;

        #[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
        pub struct Request {
            pub user_agent: String,
            pub response_time: i32,
            pub timestamp: Option<DateTime<Utc>>,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
