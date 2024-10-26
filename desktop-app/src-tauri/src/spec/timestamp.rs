use chrono::{Timelike, Utc};

pub fn to_pb_timestamp(value: chrono::DateTime<Utc>) -> pbjson_types::Timestamp {
    pbjson_types::Timestamp {
        seconds: value.timestamp(),
        nanos: value.nanosecond() as i32,
    }
}
