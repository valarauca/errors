
//! Errors
//!
//! Is a relatively simple crate for ignoring errors.
//!
//! Look, you don't give a shit about errors. They're just
//! getting in the way of you writing your application
//! layer glue code that is going to throw the error into
//! a logger somewhere.
//!
//! You've likely never really thought about errors. You just
//! kind of `format!("{:?}",` into logger statements.
//!
//! Well this is the crate for you!
//!
//! ---
//!
//!
//! Consider the following
//!
//! ```no_test
//! use std::path::Path;
//! use std::fs::File;
//!
//! pub fn open_file<P: AsRef<Path>>(path: P) -> Result<File,String> {
//!     errorf!(File::create(path), "failed to open file: {:?}", path)
//! }
//! ```
//!
//! This presents an error message that looks like:
//!
//!```ignore
//!Error Occured: failed to open file: "awesome file"
//!Module: openfile
//!Location: src/openfile.rs:20:9
//!Error: Os { code: 2, kind: NotFound, message: "No such file or directory" }
//!```

#[macro_export]
macro_rules! returnf {
    ($do: expr, $($arg:expr),*) => {
        {
            match errorf!($do, $($arg),*) {
                Ok(x) => x,
                Err(e) => return Err(e)
            }
        }
    };
}

#[macro_export]
macro_rules! exitf {
    ($do: expr; CODE: $code: expr; $($arg: expr),*) => {
        {
            match errorf!($do, $($arg),*) {
                Ok(x) => x,
                Err(ref e) => {
                    print!("{}", e);
                    ::std::process::exit($code)
                }
            }
        }
    };
    ($do: expr, $($arg:expr),*) => {
        exitf!($do; CODE: 1; $($arg),*)
    };
}

#[macro_export]
macro_rules! errorf {
    ($do:expr, $($arg:expr),* $(,)*) => {
        errorf!(@INNER $do, $($arg),*)
    };
    (@COUNTER) => {0usize};
    (@COUNTER $a: expr $(,)*) => {1usize};
    (@COUNTER $a: expr, $($b:expr),* $(,)*) => {1usize + errorf!(@COUNTER $($b),*) };
    (@INNER $do:expr, $($arg:expr),* $(,)*) => {
        {
            match $do {
                Ok(x) => Ok(x),
                Err(ref e) => {
                    // figure out the EOL character
                    const EOL: &'static str = {
                        #[cfg(target_os="windows")] { "\r\n" }
                        #[cfg(not(target_os="windows"))] { "\n" }
                    };

                    // general message framing stuf
                    let msg = match errorf!(@COUNTER $($arg),*) {
                        0 => "No user supplied data".to_string(),
                        _ => format!($($arg),*),
                    };
                    let mut error_msg = format!("Error Occured: {}{}Location: {}:{}:{}{}Error: ",
                        msg, EOL,
                        file!(), column!(), line!(), EOL);
                    
                    // normalize the error message (in case of multi-line stuff)
                    let mut msg = format!("{:?}", e);
                    if msg.lines().count() > 1 {
                        error_msg.push_str(EOL);
                        msg = msg.lines()
                            .map(|line| ["     ", line, EOL])
                            .fold(String::with_capacity(4096), |buf, line| -> String {
                                let mut buf = buf;
                                buf.push_str(line[0]);
                                buf.push_str(line[1]);
                                buf.push_str(line[2]);
                                buf
                            });
                    }
                    error_msg.push_str(&msg);
                    error_msg.push_str(EOL);
                       
                    // are we grabbing code?
                    match option_env!("ERRORS_CRATE_CODE") {
                        Option::Some(arg) => {
                            // figure out how much we take on either side
                            let size: u32 = match u32::from_str_radix(arg, 10) {
                                Ok(size) => size,
                                Err(e) => panic!("invalid value for `ERRORS_CRATE_CODE_SIZE`. value:({}) couldnt parse to number. Error:({:?})", arg, e)
                            };
                            error_msg.push_str("CODE: ");
                            error_msg.push_str(concat!(file!(), ":", column!(), ":", line!()));
                            error_msg.push_str(EOL);

                            // load the local file
                            let file: &'static str = include_str!(concat!("../", file!()));

                            // safely create offsets
                            let upper = match line!().checked_add(size) {
                                Option::None => ::std::usize::MAX,
                                Option::Some(upper) => upper as usize
                            };
                            let lower = match line!().checked_sub(size) {
                                Option::None => 0usize,
                                Option::Some(lower) => lower as usize,
                            };

                            // collect line numbers
                            let line_nums: Vec<String> = Range{ start: lower, end: upper + 1}
                                .map(|val| val.to_string())
                                .collect();
                            match line_nums.len() {
                                0|1 => Err(err_msg),
                                _ => {
                                    let first = line_nums[0].len();
                                    let last = line_nums[ line_nums.len() - 1 ].len();

                                    // padd line numbers if needed
                                    let bump = first != last;
                                    let line_nums = line_nums.into_iter().map(|mut line| -> String {
                                        loop {
                                            if line.len() < last.len() {
                                                line.push(' ');
                                            } else {
                                                break
                                            }
                                        }
                                        line
                                    }).collect::<Vec<String>>();

                                    // just takes the parts of the file we need
                                    Err(file!().lines().enumerate()
                                        .filter_map(|(line_num,line)| -> Option<&'static str> {
                                            if line_num.ge(&lower) && line_num.le(&upper) {
                                                Some(line)
                                            } else {
                                                None
                                            }
                                        })
                                        .zip(line_nums.into_iter())
                                        .map(|(line, mut line_num)| -> String {
                                            line_num.push_str(line);
                                            line_num
                                        })
                                        .fold(error_msg, |mut buf, line| -> String {
                                            buf.push_str(&line);
                                            buf
                                        }))
                                }
                            }
                        },
                        Option::Some(_) => Err(error_msg)
                    }
                }
            }
        }
    }
}

