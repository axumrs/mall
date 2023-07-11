use crate::{pb, utils::dt};

pub struct DatetimeRange {
    pub start: chrono::DateTime<chrono::Local>,
    pub end: chrono::DateTime<chrono::Local>,
}

impl From<pb::DateRange> for DatetimeRange {
    fn from(dr: pb::DateRange) -> Self {
        let start = dt::prost2chrono(&dr.start);
        let end = dt::prost2chrono(&dr.end);
        Self { start, end }
    }
}

impl Into<pb::DateRange> for DatetimeRange {
    fn into(self) -> pb::DateRange {
        let start = dt::chrono2prost(&self.start);
        let end = dt::chrono2prost(&self.end);
        pb::DateRange { start, end }
    }
}
