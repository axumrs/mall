use chrono::{Local, TimeZone};
pub fn prost2chrono(ts: &Option<prost_types::Timestamp>) -> chrono::DateTime<Local> {
    match ts {
        Some(ts) => Local.timestamp_opt(ts.seconds, ts.nanos as u32).unwrap(),
        None => chrono::Local::now(),
    }
}

pub fn chrono2prost(ts: &chrono::DateTime<Local>) -> Option<prost_types::Timestamp> {
    Some(prost_types::Timestamp {
        seconds: ts.timestamp(),
        nanos: 0,
    })
}
