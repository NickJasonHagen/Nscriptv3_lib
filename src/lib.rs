pub use std::collections::HashMap;
pub use std::io::{self, Write,Read};
use std::fs::File;
pub use encoding_rs::UTF_8;
use hex::FromHex;
pub use colored::Colorize;
mod incl {
    pub mod interpreter;
    pub mod formatter;
    pub mod basicfunctions;
    pub mod httpnc;
    pub mod tcp;
    pub mod nscript3d;
}
pub use incl::interpreter::*;
pub use incl::formatter::*;
pub use incl::basicfunctions::*;
pub use incl::httpnc::*;
pub use incl::tcp::*;
pub use incl::nscript3d::*;
pub use std::sync::{mpsc, Arc, Mutex};
pub use std::thread;
pub use std::env;
pub use std::fs;
pub use std::clone::Clone;
pub use std::path::Path;
pub use std::process::Command;
pub use rand::seq::SliceRandom;
pub use std::time::Duration;
pub use rand::Rng;
pub use std::net::{TcpListener, TcpStream,Shutdown};
pub use std::time::Instant;
pub use chrono::{Datelike, Timelike};
pub use base64::prelude::*;
pub const NC_PROGRAM_DIR: &str = env!("CARGO_MANIFEST_DIR");


pub const NSCRIPT_VERSION: &'static str = "3.004";

#[cfg(windows)]
const MACRO_OS: &'static str = "Windows";
#[cfg(not(windows))]
pub const MACRO_OS: &'static str = "Unix";
pub const NC_SERVER_ADDRESS: &str = "0.0.0.0";
pub const NC_SERVER_PORT: u16 = 8088;
#[cfg(not(windows))]
pub const NC_SERVER_ROOT: &str = "./public/";
#[cfg(windows)]
pub const NC_SERVER_ROOT: &str = ".\\public\\";
#[cfg(not(windows))]
pub const NC_SCRIPT_DIR: &str = "./";
#[cfg(windows)]
pub const NC_SCRIPT_DIR: &str = ".\\";
#[cfg(windows)]
const NC_LINE_ENDING: &'static str = "\n";
#[cfg(not(windows))]
pub const NC_LINE_ENDING: &'static str = "\n";

