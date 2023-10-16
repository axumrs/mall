use crate::pb;

use super::{U32, U64};

pub struct U32Range {
    pub min: U32,
    pub max: U32,
}

impl From<pb::U32Range> for U32Range {
    fn from(r: pb::U32Range) -> Self {
        Self {
            min: r.min.into(),
            max: r.max.into(),
        }
    }
}

impl Into<pb::U32Range> for U32Range {
    fn into(self) -> pb::U32Range {
        pb::U32Range {
            min: self.min.into(),
            max: self.max.into(),
        }
    }
}

// ----------------------------------------------------------------

pub struct U64Range {
    pub min: U64,
    pub max: U64,
}

impl From<pb::U64Range> for U64Range {
    fn from(r: pb::U64Range) -> Self {
        Self {
            min: r.min.into(),
            max: r.max.into(),
        }
    }
}

impl Into<pb::U64Range> for U64Range {
    fn into(self) -> pb::U64Range {
        pb::U64Range {
            min: self.min.into(),
            max: self.max.into(),
        }
    }
}
