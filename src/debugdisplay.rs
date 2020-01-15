use std::borrow::Cow;
use std::error;
use std::fmt;
use std::ops::Deref;

/// MessageWrapper is a strange type
/// It works much like the `Wrapper<T>` along with `BasicType`
/// to ensure better wild card generic stability.
pub struct MessageWrapper<T>(pub T);
impl From<&str> for MessageWrapper<Box<str>> {
    #[inline(always)]
    fn from(arg: &str) -> Self {
        MessageWrapper(String::from(arg).into_boxed_str())
    }
}
impl From<String> for MessageWrapper<Box<str>> {
    #[inline(always)]
    fn from(arg: String) -> Self {
        MessageWrapper(arg.into_boxed_str())
    }
}
impl From<::std::fmt::Arguments<'_>> for MessageWrapper<Box<str>> {
    #[inline(always)]
    fn from(arg: ::std::fmt::Arguments<'_>) -> Self {
        MessageWrapper(format!("{:?}", arg).into_boxed_str())
    }
}
impl From<Box<str>> for MessageWrapper<Box<str>> {
    #[inline(always)]
    fn from(arg: Box<str>) -> Self {
        MessageWrapper(arg)
    }
}
impl From<&Cow<'_, str>> for MessageWrapper<Box<str>> {
    #[inline(always)]
    fn from(arg: &Cow<'_, str>) -> Self {
        MessageWrapper(String::from(arg.as_ref()).into_boxed_str())
    }
}
impl From<&String> for MessageWrapper<Box<str>> {
    #[inline(always)]
    fn from(arg: &String) -> Self {
        MessageWrapper(arg.clone().into_boxed_str())
    }
}
impl From<&&str> for MessageWrapper<Box<str>> {
    #[inline(always)]
    fn from(arg: &&str) -> Self {
        MessageWrapper(String::from(*arg).into_boxed_str())
    }
}
impl From<&&Cow<'_, str>> for MessageWrapper<Box<str>> {
    #[inline(always)]
    fn from(arg: &&Cow<'_, str>) -> Self {
        MessageWrapper(String::from(arg.as_ref()).into_boxed_str())
    }
}
impl From<&&String> for MessageWrapper<Box<str>> {
    #[inline(always)]
    fn from(arg: &&String) -> Self {
        MessageWrapper((*arg).clone().into_boxed_str())
    }
}

/// Message type effectively doesn't exist, it just wraps around a string
pub struct Message(pub Box<str>);
impl From<MessageWrapper<Box<str>>> for Message {
    fn from(s: MessageWrapper<Box<str>>) -> Message {
        Message(s.0)
    }
}
impl From<Box<str>> for Message {
    fn from(s: Box<str>) -> Message {
        Message(s)
    }
}
