use chrono::Utc;

pub type Date = chrono::DateTime<Utc>;

pub fn now() -> Date {
    Utc::now().into()
}
