use chrono::prelude::DateTime;
use chrono::Utc;

pub const DATE_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.3fZ";

pub fn get_readable_timestamp() -> String {
    DateTime::<Utc>::from(Utc::now())
        .format(DATE_FORMAT)
        .to_string()
}
