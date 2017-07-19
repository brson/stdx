
// Extended std prelude.

// Re-export all modules from std to allow use stdx::*;
pub use std::*;

// Export the most commonly used parts of std that are not in the std prelude.

pub use std::any::{Any, TypeId};
pub use std::ascii::AsciiExt;
pub use std::borrow::{Borrow, BorrowMut, ToOwned, Cow};
pub use std::cell::{Cell, RefCell};
pub use std::cmp::{min, max};
pub use std::clone::Clone;
pub use std::collections::{HashMap, HashSet, VecDeque};
pub use std::default::Default;
pub use std::error::Error;
pub use std::ffi::OsString;
pub use std::fmt::{Debug, Display};
pub use std::fs::File;
pub use std::hash::Hash;
pub use std::io::{BufRead, Read, Seek, SeekFrom, Write};
pub use std::iter::{FromIterator, IntoIterator, Iterator};
pub use std::ops::{Deref, DerefMut};
pub use std::path::{Path, PathBuf};
pub use std::rc::Rc;
pub use std::str::FromStr;
pub use std::sync::{Arc, Mutex, RwLock};
pub use std::time::{Duration};

// Re-export included crates and the most important types.

pub extern crate bitflags;
pub extern crate byteorder;
pub extern crate chrono;
pub extern crate clap;
pub extern crate error_chain;
pub extern crate flate2;
pub extern crate fnv;
pub extern crate itertools;
pub extern crate lazy_static;
pub extern crate libc;
pub extern crate log;
pub extern crate memmap;
pub extern crate ndarray;
pub extern crate num;
pub extern crate num_cpus;
pub extern crate rand;
pub extern crate rayon;
pub extern crate regex;
pub extern crate reqwest;
pub extern crate semver;

pub extern crate serde;
pub use serde::{Deserialize, Serialize};
pub use serde::de::DeserializeOwned;

pub extern crate serde_json;
pub extern crate tar;
pub extern crate tempdir;
pub extern crate threadpool;
pub extern crate toml;
pub extern crate url;
pub extern crate walkdir;

// Supplemental dependencies.

pub extern crate env_logger;
