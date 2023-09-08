// values.rs

use chrono::{NaiveDate, NaiveTime, NaiveDateTime, DateTime, Utc, TimeZone };
use std::collections::{ HashMap };
use std::iter::{ IntoIterator };
use std::hash::{ Hash, Hasher };
use rust_decimal::{ Decimal };
use crate::blobs::{ Blob };


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Id {
    ident: String,
    metadata: Option<String>,
}
impl Id {
    pub fn new(ident: impl AsRef<str>) -> Self {
        let ident = ident.as_ref().to_string();
        Self { ident, metadata: None }
    }
    pub fn with_metadata(ident: impl AsRef<str>, metadata: impl AsRef<str>) -> Self {
        let ident = ident.as_ref().to_string();
        let metadata = metadata.as_ref().to_string();

        Self { ident, metadata: Some(metadata) }
    }

    #[inline] pub fn ident(&self) -> &str { &self.ident }
    #[inline] pub fn metadata(&self) -> Option<&String> { self.metadata.as_ref() }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Value {
    Bool(bool),
    Int(i64),
    Float(f64),
    #[cfg_attr(feature = "serde", serde(with = "rust_decimal::serde::arbitrary_precision"))]
    Decimal(Decimal),
    String(String),
    Date(NaiveDate),
    Time(NaiveTime),
    DateTime(DateTime<Utc>),
    Binary(Blob),
    List(Vec<Box<Self>>),
    Struct(HashMap<Id, Box<Self>>),
}
impl Value {
    pub fn new_struct() -> Self { Self::Struct(Default::default()) }
    pub fn new_list() -> Self { Self::List(Default::default()) }
    pub fn struct_from_entries(entries: impl IntoIterator<Item = (Id, Self)>) -> Self {
        let entries = entries.into_iter().map(|(k, v)| (k, Box::new(v))).collect();
        Self::Struct(entries)
    }
    pub fn list_from_iter(items: impl IntoIterator<Item = Self>) -> Self {
        let items = items.into_iter().map(|i| Box::new(i)).collect();
        Self::List(items)
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Bool(ref value) => matches!(other, Self::Bool(ref o) if o == value),
            Self::Int(ref value) => matches!(other, Self::Int(ref o) if o == value),
            Self::Float(ref value) => {
                let triplet = decode_bit_triplet(*value);
                matches!(other, Self::Float(ref o) if decode_bit_triplet(*o) == triplet)
            },
            Self::Decimal(ref value) => matches!(other, Self::Decimal(ref o) if o == value),
            Self::String(ref value) => matches!(other, Self::String(ref o) if o == value),
            Self::Date(ref value) => matches!(other, Self::Date(ref o) if o == value),
            Self::Time(ref value) => matches!(other, Self::Time(ref o) if o == value),
            Self::DateTime(ref value) => matches!(other, Self::DateTime(ref o) if o == value),
            Self::List(ref value) => matches!(other, Self::List(ref o) if o == value),
            Self::Struct(ref value) => matches!(other, Self::Struct(ref o) if o == value),
            Self::Binary(ref value) => matches!(other, Self::Binary(ref o) if o == value),
        }
    }
}
impl Eq for Value {}
impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Bool(ref value) => value.hash(state),
            Self::Int(ref value) => value.hash(state),
            Self::Float(ref value) => decode_bit_triplet(*value).hash(state),
            Self::Decimal(ref value) => value.hash(state),
            Self::String(ref value) => value.hash(state),
            Self::Date(ref value) => value.hash(state),
            Self::Time(ref value) => value.hash(state),
            Self::DateTime(ref value) => value.hash(state),
            Self::List(ref value) => value.hash(state),
            Self::Binary(ref value) => value.hash(state),
            Self::Struct(ref value) => for entry in value {
                entry.hash(state);
            },
        }
    }
}

fn decode_bit_triplet(value: f64) -> (u64, i16, i8) {
    use std::mem::{ transmute };

    // Convert to mantissa-exponent-sign triplet
    let bits: u64 = unsafe { transmute(value) };

    let sign: i8 = if bits >> 63 == 0 { 1 } else { 0 };
    let mut exponent = ((bits >> 52) & 0x7FF) as i16;
    let mantissa = match exponent {
        0 => (bits & 0xFFFFFFFFFFFFF) << 1,
        _ => (bits & 0xFFFFFFFFFFFFF) | 0x1000000000000,
    };

    exponent -= 1023 + 52;
    (mantissa, exponent, sign)
}


macro_rules! impl_from_numeric {
    ($id:ident => $($t:ty),*) => {
        $(impl From<$t> for $crate::values::Value {
            fn from(value: $t) -> Self { Self::$id(value.into()) }
        })*
    };
}
macro_rules! impl_as_numeric {
    ($id:ident => $($t:ty),*) => {
        $(impl From<$t> for $crate::values::Value {
            fn from(value: $t) -> Self { Self::$id(value as i64) }
        })*
    };
}

impl_from_numeric!(Int => i8, i16, i32, i64, u8, u16, u32);
impl_as_numeric!(Int => i128, u64, u128);
impl_from_numeric!(Float => f32, f64);

impl From<bool> for Value {
    fn from(value: bool) -> Self { Self::Bool(value) }
}
impl From<Decimal> for Value {
    fn from(value: Decimal) -> Self { Self::Decimal(value) }
}
impl From<&str> for Value {
    fn from(value: &str) -> Self {
        let value = value.to_string();
        Self::String(value)
    }
}
impl From<String> for Value {
    fn from(value: String) -> Self { Self::String(value) }
}
impl From<NaiveDate> for Value {
    fn from(value: NaiveDate) -> Self { Self::Date(value) }
}
impl From<NaiveTime> for Value {
    fn from(value: NaiveTime) -> Self { Self::Time(value) }
}
impl From<NaiveDateTime> for Value {
    fn from(value: NaiveDateTime) -> Self { Self::DateTime(value.and_utc()) }
}
impl<T: TimeZone> From<DateTime<T>> for Value {
    fn from(value: DateTime<T>) -> Self {
        let value = value.with_timezone(&Utc);
        Self::DateTime(value)
    }
}
impl<T: Into<Blob>> From<T> for Value {
    fn from(value: T) -> Self { Self::Binary(value.into()) }
}
