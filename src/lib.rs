extern crate serde;
extern crate serde_json;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

use std::error;
use std::fmt;

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
    inner: std::sync::Arc<std::sync::Mutex<ErrorInner>>,
}

struct ErrorInner {
    message: Box<str>,
    root_cause: Option<BasicType>,
    key_value: KeyValue,
}
impl ErrorInner {
    fn msg<'a>(&'a self) -> &'a str {
        self.message.as_ref()
    }
}


impl Serialize for Err {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_struct("Error", 3)?;
        let elf = self.inner.as_ref().lock().unwrap();
        s.serialize_field("message", &elf.message)?;
        s.serialize_field("existing_error", &elf.root_cause)?;
        s.serialize_field("info", &elf.key_value)?;
        s.end()
    }
}
impl error::Error for Err {
    fn description(&self) -> &str {
        "Description is soft depreciated. Therefore it is not directly supported by this crate"
    }
}
impl fmt::Display for Err {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}
impl fmt::Debug for Err {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let elf = self.inner.as_ref().lock().unwrap();

        let mut info = f.debug_struct("Error");
        info.field("message", &elf.message);
        if let Option::Some(ref root_cause) = elf.root_cause {
            info.field("existing_error", root_cause);
        }
        if elf.key_value.len() > 0 {
            info.field("info", &elf.key_value);
        }
        info.finish()
    }
}
impl Default for Err {
    #[inline(always)]
    fn default() -> Err {
        Err {
            inner: std::sync::Arc::new(std::sync::Mutex::new( ErrorInner {
                message: String::new().into_boxed_str(),
                root_cause: None,
                key_value: KeyValue::default(),
            }))
        }
    }
}

macro_rules! try_err {
    ($expression: expr => $message: expr => $err: expr) => {
        match $expression {
            Ok(x) => x,
            Err(e) => {
                Err($err
                    .note("position", format!("{}:{}:{}", file!, line!(), column!()))
                    .note("module", module_path!())
                    .err(e, $message))
            }
        }
    };
    ($expression: expr => $message: expr) => {
        match $expression {
            Ok(x) => x,
            Err(e) => {
                Err(self::Err::default()
                    .note("position", format!("{}:{}:{}", file!, line!(), column!()))
                    .note("module", module_path!())
                    .err(e, $message))
            }
        }
    };
}


impl Err {

    /// error can work with most error/message formats
    pub fn err<A, B>(&self, err: A, message: &'static str) -> Self
    where
        Wrapper<B>: From<A>,
        BasicType: From<Wrapper<B>>,
    {
        let item = self.clone();
        {
            let mut e = item.inner.as_ref().lock().unwrap();
            e.message = Message::from(MessageWrapper::from(message)).0;
            e.root_cause = Some(BasicType::from(Wrapper::<B>::from(err)));
        }
        item
    }

    /// appends kv data to an existing error
    pub fn note<A, B>(&self, key: &'static str, value: A) -> Self
    where
        Wrapper<B>: From<A>,
        BasicType: From<Wrapper<B>>,
    {
        let item = self.clone();
        {
            let mut s = item.inner.as_ref().lock().unwrap();
            s.key_value.insert(key, value);
        }
        item
    }

    /// serializes the data into a compact JSON representation
    pub fn to_json(&self) -> ::serde_json::Result<String> {
        ::serde_json::to_string(self)
    }

    /// serializes the data into a human readable JSON representation
    pub fn to_json_pretty(&self) -> ::serde_json::Result<String> {
        ::serde_json::to_string_pretty(self)
    }
}
