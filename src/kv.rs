use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::collections::HashMap;
use std::fmt;
use std::io;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};

use super::serde::ser::SerializeStruct;
use super::serde::{Serialize, Serializer};

/// KeyValue is a specialized type to hold key-value data associated with
/// an error.
#[derive(Clone, Default, Serialize)]
pub struct KeyValue(HashMap<&'static str, BasicType>);
impl KeyValue {
    /// populates a value to within the map of values. Keys must be unique, the program
    /// will panic if this is not respected.
    ///
    /// funciton is purposefully tagged `inline(always)` the massive number of generic
    /// combinations it (purposefully) offers means the debug information it can
    /// potentially generate is HUGE.
    /// Therefore it being inlined, and its debug information being dropped it best
    /// for everyone.
    #[inline(always)]
    pub fn insert<A>(&mut self, key: &'static str, arg: A)
    where
        BasicType: From<A>,
    {
        self.0.insert(key, BasicType::from(arg));
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    fn count_fields(&self) -> usize {
        self.0.len()
    }
}
impl fmt::Display for KeyValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}
impl fmt::Debug for KeyValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut info = f.debug_struct("ErrorInfo");
        let mut v = self
            .0
            .iter()
            .map(SortValue::from)
            .collect::<Vec<SortValue<'_>>>();
        v.sort();
        for item in v {
            info.field(*item.key, item.value);
        }
        info.finish()
    }
}

struct SortValue<'a> {
    key: &'a &'static str,
    value: &'a BasicType,
}
impl<'a> From<(&'a &'static str, &'a BasicType)> for SortValue<'a> {
    fn from(tup: (&'a &'static str, &'a BasicType)) -> SortValue<'a> {
        SortValue {
            key: tup.0,
            value: tup.1,
        }
    }
}
impl<'a> PartialEq for SortValue<'a> {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.key.eq(other.key)
    }
}
impl<'a> Eq for SortValue<'a> {}
impl<'a> PartialOrd for SortValue<'a> {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.key.partial_cmp(other.key)
    }
}
impl<'a> Ord for SortValue<'a> {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(other.key)
    }
}
macro_rules! from_primative_to_basic {
    (@WRAPPER $from_name: ident; $container: path; $variant: ident) => {
        impl From<$from_name> for BasicType {
            #[inline(always)]
            fn from(arg: $from_name) -> Self {
                Self::$variant($container(arg))
            }
        }
        impl<'a> From<&'a $from_name> for BasicType {
            #[inline(always)]
            fn from(arg: &$from_name) -> Self {
                Self::$variant($container(arg.clone()))
            }
        }
    };
    ($from_name: ty; $variant: ident) => {
        impl From<$from_name> for BasicType {
            #[inline(always)]
            fn from(arg: $from_name) -> Self {
                Self::$variant(arg)
            }
        }
        impl<'a> From<&'a $from_name> for BasicType {
            #[inline(always)]
            fn from(arg: &$from_name) -> Self {
                Self::$variant(arg.clone())
            }
        }
    };
}

/// BasicType is used as a "relatively" efficient way to store
/// multiple primative types without having to preform extra
/// allocations.
#[derive(Clone)]
pub enum BasicType {
    Bool(bool),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    F32(f32),
    F64(f64),
    StaticStr(&'static str),
    IP(IpAddr),
    Socket(SocketAddr),
    Dur(Duration),
    Inst(Instant),
    SysTime(SystemTime),
    String(String),
    IOError(Arc<io::Error>),
}
impl fmt::Debug for BasicType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bool(ref item) => write!(f, "{:?}", item),
            Self::I8(ref item) => write!(f, "{:?}", item),
            Self::I16(ref item) => write!(f, "{:?}", item),
            Self::I32(ref item) => write!(f, "{:?}", item),
            Self::I64(ref item) => write!(f, "{:?}", item),
            Self::I128(ref item) => write!(f, "{:?}", item),
            Self::U8(ref item) => write!(f, "{:?}", item),
            Self::U16(ref item) => write!(f, "{:?}", item),
            Self::U32(ref item) => write!(f, "{:?}", item),
            Self::U64(ref item) => write!(f, "{:?}", item),
            Self::U128(ref item) => write!(f, "{:?}", item),
            Self::F32(ref item) => write!(f, "{:?}", item),
            Self::F64(ref item) => write!(f, "{:?}", item),
            Self::StaticStr(ref item) => write!(f, "{:?}", item),
            Self::String(ref item) => write!(f, "{:?}", item),
            Self::IP(IpAddr::V4(ref ip)) => write!(f, "{:?}", ip),
            Self::IP(IpAddr::V6(ref ip)) => write!(f, "{:?}", ip),
            Self::Socket(SocketAddr::V4(ref sock)) => write!(f, "{:?}", sock),
            Self::Socket(SocketAddr::V6(ref sock)) => write!(f, "{:?}", sock),
            Self::Dur(ref item) => write!(f, "{:?}", item),
            Self::Inst(ref item) => write!(f, "{:?}", item),
            Self::SysTime(ref item) => write!(
                f,
                "{:?}",
                item.duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap_or_else(|_| Duration::new(0, 0))
            ),
            Self::IOError(ref err) => write!(f, "{:?}", err),
        }
    }
}
impl Serialize for BasicType {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Bool(ref item) => s.serialize_bool(*item),
            Self::I8(ref item) => s.serialize_i8(*item),
            Self::I16(ref item) => s.serialize_i16(*item),
            Self::I32(ref item) => s.serialize_i32(*item),
            Self::I64(ref item) => s.serialize_i64(*item),
            Self::I128(ref item) => s.serialize_i128(*item),
            Self::U8(ref item) => s.serialize_u8(*item),
            Self::U16(ref item) => s.serialize_u16(*item),
            Self::U32(ref item) => s.serialize_u32(*item),
            Self::U64(ref item) => s.serialize_u64(*item),
            Self::U128(ref item) => s.serialize_u128(*item),
            Self::F32(ref item) => s.serialize_f32(*item),
            Self::F64(ref item) => s.serialize_f64(*item),
            Self::StaticStr(ref item) => s.serialize_str(item),
            Self::String(ref item) => s.serialize_str(item),
            Self::IP(IpAddr::V4(ref ip)) => s.collect_str(ip),
            Self::IP(IpAddr::V6(ref ip)) => s.collect_str(ip),
            Self::Socket(SocketAddr::V4(ref sock)) => s.collect_str(sock),
            Self::Socket(SocketAddr::V6(ref sock)) => s.collect_str(sock),
            Self::Dur(ref item) => s.serialize_str(&format!("{:?}", item)),
            Self::Inst(ref item) => s.serialize_str(&format!("{:?}", item)),
            Self::SysTime(ref item) => s.serialize_str(&format!(
                "{:?}",
                item.duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap_or_else(|_| Duration::new(0, 0))
            )),
            Self::IOError(ref err) => s.collect_str(err),
        }
    }
}
from_primative_to_basic!(bool;Bool);
from_primative_to_basic!(i8;I8);
from_primative_to_basic!(i16;I16);
from_primative_to_basic!(i32;I32);
from_primative_to_basic!(i64;I64);
from_primative_to_basic!(i128;I128);
from_primative_to_basic!(u8;U8);
from_primative_to_basic!(u16;U16);
from_primative_to_basic!(u32;U32);
from_primative_to_basic!(u64;U64);
from_primative_to_basic!(u128;U128);
from_primative_to_basic!(f32;F32);
from_primative_to_basic!(f64;F64);
from_primative_to_basic!(IpAddr;IP);
from_primative_to_basic!(SocketAddr;Socket);
from_primative_to_basic!(Duration;Dur);
from_primative_to_basic!(Instant;Inst);
from_primative_to_basic!(SystemTime;SysTime);
from_primative_to_basic!(String;String);
from_primative_to_basic!(&'static str; StaticStr);
from_primative_to_basic!(@WRAPPER Ipv4Addr; IpAddr::V4; IP);
from_primative_to_basic!(@WRAPPER Ipv6Addr; IpAddr::V6; IP);
from_primative_to_basic!(@WRAPPER SocketAddrV4; SocketAddr::V4; Socket);
from_primative_to_basic!(@WRAPPER SocketAddrV6; SocketAddr::V6; Socket);

impl From<std::io::Error> for BasicType {
    fn from(arg: std::io::Error) -> Self {
        Self::IOError(Arc::new(arg))
    }
}
