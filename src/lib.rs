extern crate backtrace;
extern crate serde;
extern crate serde_json;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

use std::error;
use std::fmt;

mod frame;
use self::frame::{build_backtrace, Frames};
mod kv;
use self::kv::KeyValue;
pub use self::kv::{BasicType, Wrapper};
mod debugdisplay;
pub use self::debugdisplay::{Message, MessageWrapper};

/// Err is a heavy, but complete custom error type system.
///
/// Err produces rich backtrace information in a format of
/// your want, in a relatively sane fashion.
#[derive(Clone)]
pub struct Err {
    backtrace: Frames,
    message: String,
    root_cause: Option<Box<Err>>,
    key_value: KeyValue,
}
impl Serialize for Err {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_struct("Error", self.count_fields())?;
        s.serialize_field("stack", &self.backtrace)?;
        s.serialize_field("message", &self.message)?;
        if let Option::Some(ref root_cause) = &self.root_cause {
            s.serialize_field("existing_error", root_cause)?;
        }
        if self.key_value.len() > 0 {
            s.serialize_field("info", &self.key_value)?;
        }
        s.end()
    }
}
impl error::Error for Err {
    fn description(&self) -> &str {
        "Description is soft depreciated. Therefore it is not directly supported by this crate"
    }
    fn source<'a>(&'a self) -> Option<&'a (dyn error::Error + 'static)> {
        // amusingly this code breaks if you listen to clippy
        #[allow(clippy::match_ref_pats)]
        match &self.root_cause {
            &Option::None => None,
            &Option::Some(ref err) => Some(err),
        }
    }
    fn cause<'a>(&'a self) -> Option<&'a (dyn error::Error + 'static)> {
        // amusingly this code breaks if you listen to clippy
        #[allow(clippy::match_ref_pats)]
        match &self.root_cause {
            &Option::None => None,
            &Option::Some(ref err) => Some(err),
        }
    }
}
impl fmt::Display for Err {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}
impl fmt::Debug for Err {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut info = f.debug_struct("Error");
        info.field("stack", &self.backtrace);
        info.field("message", &self.message);
        if let Option::Some(ref root_cause) = &self.root_cause {
            info.field("existing_error", root_cause);
        }
        if self.key_value.len() > 0 {
            info.field("info", &self.key_value);
        }
        info.finish()
    }
}
impl Default for Err {
    #[inline(always)]
    fn default() -> Err {
        Err::new_with_skip(2)
    }
}
impl Err {
    /// new creates a new error value.
    ///
    /// The `message` argument maybe a little confusing in truth it
    /// accepts _many_ types. The full list can be found on
    /// `MessageWrapper<B>`'s page. As what ever it be created from
    /// is a type that a string can be created from
    #[inline(always)]
    pub fn new<A, B>(message: A) -> Self
    where
        MessageWrapper<B>: From<A>,
        Message: From<MessageWrapper<B>>,
    {
        let mut e = Err::default();
        e.message = Message::from(MessageWrapper::from(message)).into();
        e
    }

    /// new with error creates an error with a root cause
    #[inline(always)]
    pub fn new_with_error<A, B>(err: Err, message: A) -> Self
    where
        MessageWrapper<B>: From<A>,
        Message: From<MessageWrapper<B>>,
    {
        let mut e = Err::new(message);
        e.root_cause = Some(Box::new(err));
        e
    }

    /// appends a note to an error.
    #[inline(always)]
    pub fn note<A, B>(self, key: &'static str, value: A) -> Self
    where
        Wrapper<B>: From<A>,
        BasicType: From<Wrapper<B>>,
    {
        let mut s = self;
        s.key_value.insert(key, value);
        s
    }

    /// serializes the data into a compact JSON representation
    pub fn to_json(&self) -> ::serde_json::Result<String> {
        ::serde_json::to_string(self)
    }

    /// serializes the data into a human readable JSON representation
    pub fn to_json_pretty(&self) -> ::serde_json::Result<String> {
        ::serde_json::to_string_pretty(self)
    }

    #[inline(never)]
    fn new_with_skip(skip: usize) -> Err {
        let skip = if skip < 2 { 2 } else { skip };
        let root_cause = None;
        let key_value = KeyValue::default();
        let backtrace = build_backtrace(skip, 1000);
        Err {
            message: String::with_capacity(0),
            backtrace,
            root_cause,
            key_value,
        }
    }

    fn count_fields(&self) -> usize {
        let mut fields = 2usize;
        if self.key_value.len() > 0 {
            fields += 1;
        }
        if self.root_cause.is_some() {
            fields += 1;
        }
        fields
    }
}
