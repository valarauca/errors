use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::collections::HashMap;
use std::fmt;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};

use super::serde::ser::SerializeStruct;
use super::serde::{Serialize, Serializer};

/// KeyValue is a specialized type to hold key-value data associated with
/// an error.
#[derive(Clone, Default)]
pub struct KeyValue {
    data: HashMap<&'static str, BasicType>,
}
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
    pub fn insert<A, B>(&mut self, key: &'static str, arg: A)
    where
        Wrapper<B>: From<A>,
        BasicType: From<Wrapper<B>>,
    {
        match self.data.get(key) {
            Option::Some(ref value) => {
                panic!("Err: cannot insert key:'{}' into error map. key already exists with value:'{:?}'", key, value);
            }
            Option::None => {}
        };
        self.data
            .insert(key, BasicType::from(Wrapper::<B>::from(arg)));
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    fn count_fields(&self) -> usize {
        self.data.len()
    }
}
impl Serialize for KeyValue {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_struct("ErrorInfo", self.count_fields())?;

        // keys will be ordered
        let mut v = self
            .data
            .iter()
            .map(SortValue::from)
            .collect::<Vec<SortValue<'_>>>();
        v.sort();

        // glob everything together
        for item in v {
            s.serialize_field(*item.key, item.value)?;
        }
        s.end()
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
            .data
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

/// Wrapper is a very strange type which exists for specialization hacks.
///
/// Wrapper has a generic parameter `T` which means we can implement things
/// for `Wrapper<T>`, or if we're fun, `Wrapper<f32>` (for example).
///
/// It means our implementations can be specialized for each input type.
/// yet, we can still expose interfaces which bind to `From<T>`.
pub struct Wrapper<T>(pub T);
impl<'a, D: fmt::Debug + Clone + 'static> From<&'a D>
    for Wrapper<(&'static str, Arc<dyn fmt::Debug>)>
{
    #[inline(always)]
    fn from(arg: &'a D) -> Self {
        Wrapper((::std::any::type_name::<D>(), Arc::new(arg.clone())))
    }
}
impl From<Wrapper<(&'static str, Arc<dyn fmt::Debug>)>> for BasicType {
    #[inline(always)]
    fn from(arg: Wrapper<(&'static str, Arc<dyn fmt::Debug>)>) -> Self {
        Self::Debug((arg.0).0, (arg.0).1)
    }
}
macro_rules! from_primative_to_basic {
    (@WRAPPER $from_name: ident; $container: path; $variant: ident) => {
        impl From<$from_name> for Wrapper<$from_name> {
            #[inline(always)]
            fn from(arg: $from_name) -> Self {
                Self(arg)
            }
        }
        impl<'a> From<&'a $from_name> for Wrapper<$from_name> {
            #[inline(always)]
            fn from(arg: &$from_name) -> Self {
                Self(arg.clone())
            }
        }
        impl From<Wrapper<$from_name>> for BasicType {
            #[inline(always)]
            fn from(arg: Wrapper<$from_name>) -> Self {
                Self::$variant($container(arg.0))
            }
        }
    };
    ($from_name: ty; $variant: ident) => {
        impl From<$from_name> for Wrapper<$from_name> {
            #[inline(always)]
            fn from(arg: $from_name) -> Self {
                Self(arg)
            }
        }
        impl<'a> From<&'a $from_name> for Wrapper<$from_name> {
            #[inline(always)]
            fn from(arg: &$from_name) -> Self {
                Self(arg.clone())
            }
        }
        impl From<Wrapper<$from_name>> for BasicType {
            #[inline(always)]
            fn from(arg: Wrapper<$from_name>) -> Self {
                Self::$variant(arg.0)
            }
        }
    };
}

/// BasicType is used as a "relatively" efficient way to store
/// multiple primative types without having to preform extra
/// allocations.
#[derive(fmt::Debug, Clone)]
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
    Debug(&'static str, Arc<dyn fmt::Debug>),
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
            Self::Debug(ref kind, ref debug) => s.serialize_str(&format!("{}='{:?}'", kind, debug)),
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
from_primative_to_basic!(&'static str; StaticStr);
from_primative_to_basic!(@WRAPPER Ipv4Addr; IpAddr::V4; IP);
from_primative_to_basic!(@WRAPPER Ipv6Addr; IpAddr::V6; IP);
from_primative_to_basic!(@WRAPPER SocketAddrV4; SocketAddr::V4; Socket);
from_primative_to_basic!(@WRAPPER SocketAddrV6; SocketAddr::V6; Socket);

#[test]
fn test_type_type_aliasing() {
    fn do_things<A, B>(arg: A) -> BasicType
    where
        Wrapper<B>: From<A>,
        BasicType: From<Wrapper<B>>,
    {
        let wrapped: Wrapper<B> = <Wrapper<B> as From<A>>::from(arg);
        <BasicType as From<Wrapper<B>>>::from(wrapped)
    }
    match do_things(true) {
        BasicType::Bool(true) => {}
        anything => panic!("Expected a boolean, found:'{:?}'", anything),
    };
}
