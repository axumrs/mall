use serde::{Deserialize, Serialize};
use std::ops::{Add, Div, Mul, Sub};

#[derive(sqlx::Type, Clone, Copy, Deserialize, Serialize, Debug, Default)]
#[sqlx(transparent)]
#[sqlx(type_name = "u32")]
pub struct U32(i32);

impl U32 {
    pub fn new(i: i32) -> Self {
        Self(i)
    }
    /// 原始数据
    pub fn value(&self) -> i32 {
        self.0
    }
    /// 无符号数据
    pub fn unsigned(&self) -> u32 {
        self.0 as u32
    }
    /// 将“元”变成“分”
    pub fn from_yuan(i: i32) -> Self {
        Self::new(i).to_cent()
    }
    /// 将值变成“分”
    pub fn to_cent(self) -> Self {
        self * Self::new(100)
    }
    /// 将值变成“元”
    pub fn to_yuan(self) -> Self {
        self / Self::new(100)
    }
}

impl From<i32> for U32 {
    fn from(i: i32) -> Self {
        Self::new(i)
    }
}

impl Into<i32> for U32 {
    fn into(self) -> i32 {
        self.value()
    }
}

impl From<u32> for U32 {
    fn from(value: u32) -> Self {
        Self::new(value as i32)
    }
}
impl Into<u32> for U32 {
    fn into(self) -> u32 {
        self.unsigned()
    }
}

impl Add for U32 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.unsigned() + rhs.unsigned())
    }
}

impl Sub for U32 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from(self.unsigned() - rhs.unsigned())
    }
}

impl Mul for U32 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::from(self.unsigned() * rhs.unsigned())
    }
}

impl Div for U32 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::from(self.unsigned() / rhs.unsigned())
    }
}

// ----

#[derive(sqlx::Type, Clone, Copy, Deserialize, Serialize, Debug, Default)]
#[sqlx(transparent)]
#[sqlx(type_name = "u64")]
pub struct U64(i64);

impl U64 {
    pub fn new(i: i64) -> Self {
        Self(i)
    }
    pub fn value(&self) -> i64 {
        self.0
    }
    pub fn unsigned(&self) -> u64 {
        self.0 as u64
    }
}

impl From<i64> for U64 {
    fn from(i: i64) -> Self {
        Self::new(i)
    }
}

impl Into<i64> for U64 {
    fn into(self) -> i64 {
        self.value()
    }
}

impl From<u64> for U64 {
    fn from(value: u64) -> Self {
        Self::new(value as i64)
    }
}
impl Into<u64> for U64 {
    fn into(self) -> u64 {
        self.unsigned()
    }
}

impl From<U32> for U64 {
    fn from(value: U32) -> Self {
        Self::from(value.unsigned() as u64)
    }
}

impl Into<U32> for U64 {
    fn into(self) -> U32 {
        U32::from(self.unsigned() as u32)
    }
}

impl Add for U64 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.unsigned() + rhs.unsigned())
    }
}

impl Sub for U64 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from(self.unsigned() - rhs.unsigned())
    }
}

impl Mul for U64 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::from(self.unsigned() * rhs.unsigned())
    }
}

impl Div for U64 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::from(self.unsigned() / rhs.unsigned())
    }
}
