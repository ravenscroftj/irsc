#![feature(plugin, slicing_syntax, unboxed_closures)]

#[plugin]
extern crate regex_macros;
extern crate regex;

//#[plugin]
//#[no_link]
#[macro_use]
extern crate log;

pub mod server;
pub mod color;
pub mod ident;
pub mod callback;
pub mod event;
