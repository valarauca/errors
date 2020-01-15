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

macro_rules! implement {
    (@WAT $param: ident; $require: path; $name: ident; $block:block) => {
        impl<$param: $require + 'static> From<$param>
            for MessageWrapper<Box<dyn $require + 'static>>
        {
            #[inline(always)]
            fn from(arg: $param) -> Self {
                MessageWrapper(Box::new(arg))
            }
        }
        impl From<MessageWrapper<Box<dyn $require + 'static>>> for Message {
            #[inline(always)]
            fn from($name: MessageWrapper<Box<dyn $require + 'static>>) -> Message {
                Message($block)
            }
        }
    };
    (@LIFETIME $lifetime: lifetime; $type_argument: ty) => {
        impl<$lifetime> From<$type_argument> for MessageWrapper<$type_argument> {
            #[inline(always)]
            fn from(arg: $type_argument) -> Self {
                MessageWrapper(arg)
            }
        }
        impl<$lifetime> From<MessageWrapper<$type_argument>> for Message {
            #[inline(always)]
            fn from(arg: MessageWrapper<$type_argument>) -> Message {
                Message(arg.0.to_string())
            }
        }
    };
    (@TY $type_argument: ty) => {
        impl From<$type_argument> for MessageWrapper<$type_argument> {
            #[inline(always)]
            fn from(arg: $type_argument) -> Self {
                MessageWrapper(arg)
            }
        }
        impl From<MessageWrapper<$type_argument>> for Message {
            #[inline(always)]
            fn from(arg: MessageWrapper<$type_argument>) -> Message {
                Message(arg.0.to_string())
            }
        }
    };
}
implement!(@WAT E; error::Error; arg; {
    #[allow(deprecated)]
    match (arg.0.description(), arg.0.cause(), arg.0.source()) {
        (desc, Option::Some(ref cause), Option::None) if !desc.is_empty() => {
            format!("err:'{}' description:'{}' cause:'{}'", arg.0, desc, cause)
        },
        (desc, Option::None, Option::Some(ref source)) if !desc.is_empty() => {
            format!("err:'{}' description:'{}' cause:'{}'", arg.0, desc, source)
        },
        (desc, Option::Some(ref cause), Option::Some(ref source)) if !desc.is_empty() => {
            format!("err:'{}' description:'{}' cause:'{}' source:'{}'", arg.0, desc, cause, source)
        },
        (desc, Option::None, Option::None) if !desc.is_empty() => {
            format!("err:'{}' description:'{}'", arg.0, desc)
        },
        (_, Option::Some(ref cause), Option::None) => {
            format!("err:'{}' cause:'{}'", arg.0, cause)
        },
        (_, Option::None, Option::Some(ref source)) => {
            format!("err:'{}' cause:'{}'", arg.0, source)
        },
        (_, Option::Some(ref cause), Option::Some(ref source)) => {
            format!("err:'{}' cause:'{}' source:'{}'", arg.0, cause, source)
        },
        (_, Option::None, Option::None) => {
            format!("err:'{}'", arg.0)
        }
    }
});
implement!(@WAT S; AsRef<str>; arg; { 
    String::from(arg.0.as_ref().as_ref())
});
implement!(@WAT S; Deref<Target=str>; arg; {
    String::from(arg.0.as_ref().deref())
});
implement!(@WAT S; Deref<Target=String>; arg; {
    String::from(arg.0.as_ref().deref())
});
implement!(@WAT S; AsRef<String>; arg; {
    String::from(arg.0.as_ref().as_ref())
});
implement!(@WAT D; fmt::Debug; arg; { format!("{:?}", arg.0) } );
implement!(@WAT D; fmt::Display; arg; { format!("{}", arg.0) } );
implement!(@LIFETIME 'a; &'a Box<str>);
implement!(@LIFETIME 'a; &'a Cow<'a,str>);
implement!(@LIFETIME 'a; Cow<'a,str>);
implement!(@LIFETIME 'a; &'a String);
implement!(@TY Box<str>);
implement!(@TY String);

/// Message type effectively doesn't exist, it just wraps around a string
pub struct Message(String);
impl From<Box<str>> for Message {
    fn from(s: Box<str>) -> Message {
        Message(String::from(s.as_ref()))
    }
}
impl From<String> for Message {
    #[inline(always)]
    fn from(s: String) -> Message {
        Message(s)
    }
}
impl Into<String> for Message {
    #[inline(always)]
    fn into(self) -> String {
        self.0
    }
}
