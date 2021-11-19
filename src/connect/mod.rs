//! Handles data modeling and network connections for the client.

use std::borrow::Cow;

use either::{Either, Left, Right};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Serialize, Deserialize)]
pub struct Event<'a, 'b, T> {
    #[serde(borrow)]
    topic: Either<Topic<'a>, &'a str>,
    #[serde(borrow)]
    event: Either<EventType, &'b str>,
    payload: T,
    reference: uuid::Uuid,
}

impl<'a, 'b, T> Event<'a, 'b, T> {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum Topic<'a> {
    Phoenix,
    #[serde(borrow)]
    Room(Cow<'a, str>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    Heartbeat,
    Shout,
    #[serde(rename = "phx_join")]
    PhoenixJoin,
    #[serde(rename = "phx_leave")]
    PhoenixLeave,
    #[serde(rename = "phx_error")]
    PhoenixError,
    #[serde(rename = "phx_close")]
    PhoenixClose,
}
