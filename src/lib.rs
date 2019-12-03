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

/// Err is a heavy, but complete custom error type system.
///
/// Err produces rich backtrace information in a format of
/// your want, in a relatively sane fashion.
#[derive(Clone)]
pub struct Err {
    backtrace: Frames,
    root_cause: Option<Box<Err>>,
    key_value: KeyValue,
}
impl Serialize for Err {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_struct("Error", self.count_fields())?;
        s.serialize_field("stack", &self.backtrace)?;
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
impl<'a> From<&'a Err> for Err {
    #[inline(never)]
    fn from(err: &'a Err) -> Err {
        let mut e = Err::new_with_skip(3);
        e.root_cause = Some(Box::new(err.clone()));
        e
    }
}
impl Err {
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
            backtrace,
            root_cause,
            key_value,
        }
    }

    fn count_fields(&self) -> usize {
        let mut fields = 1usize;
        if self.key_value.len() > 0 {
            fields += 1;
        }
        if self.root_cause.is_some() {
            fields += 1;
        }
        fields
    }
}
