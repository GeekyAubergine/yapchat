use chrono::{DateTime, Utc};

pub trait FormatDate {
    fn short_iso(&self) -> String;
    fn datetime(&self) -> String;
    fn without_time(&self) -> String;
}

impl FormatDate for DateTime<Utc> {
    fn short_iso(&self) -> String {
        self.format("%Y-%m-%d %H:%M").to_string()
    }

    fn datetime(&self) -> String {
        self.format("%Y-%m-%d %H:%M").to_string()
    }

    fn without_time(&self) -> String {
        self.format("%Y-%m-%d").to_string()
    }
}

impl FormatDate for Option<DateTime<Utc>> {
    fn short_iso(&self) -> String {
        match self {
            Some(date) => date.short_iso(),
            None => "-".to_string(),
        }
    }

    fn datetime(&self) -> String {
        match self {
            Some(date) => date.datetime(),
            None => "-".to_string(),
        }
    }

    fn without_time(&self) -> String {
        match self {
            Some(date) => date.without_time(),
            None => "-".to_string(),
        }
    }
}
