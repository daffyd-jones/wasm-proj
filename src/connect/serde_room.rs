use std::borrow::Cow;
use std::fmt;

use lazy_static::lazy_static;
use regex::Regex;
use serde::de::{self, Visitor};
use serde::*;

lazy_static! {
    static ref ROOM_REGEX: Regex = Regex::new("^room:(.+)").unwrap();
}

pub fn serialize<'a, S>(this: &Cow<'a, str>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&format!("room:{}", this))
}

pub fn deserialize<'de, 'a, D>(deserializer: D) -> Result<Cow<'a, str>, D::Error>
where
    'de: 'a,
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(RoomVisitor)
}

struct RoomVisitor;

impl<'a> Visitor<'a> for RoomVisitor {
    type Value = Cow<'a, str>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a string that matches `^room:(.+)`")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match ROOM_REGEX.captures(v) {
            Some(cap) => Ok(cap[1].to_string().into()),
            None => Err(de::Error::invalid_value(de::Unexpected::Str(v), &self)),
        }
    }

    fn visit_borrowed_str<E>(self, v: &'a str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if ROOM_REGEX.is_match(v) {
            // We can't use the capture groups here to achieve zero-copy, so we must do it on our own.
            // Slicing the string will work for our purposes.
            Ok(v[5..].into())
        } else {
            Err(de::Error::invalid_value(de::Unexpected::Str(v), &self))
        }
    }
}
