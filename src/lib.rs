#[cfg(feature = "bitflags")] extern crate bitflags as bitflags_;
#[cfg(feature = "bitflags")] pub use bitflags_ as bitflags;

#[cfg(feature = "docopt")] extern crate docopt as docopt_;
#[cfg(feature = "docopt")] pub use docopt_ as docopt;

#[cfg(feature = "env_logger")] extern crate env_logger as env_logger_;
#[cfg(feature = "env_logger")] pub use env_logger_ as env_logger;

#[cfg(feature = "itertools")] extern crate itertools as itertools_;
#[cfg(feature = "itertools")] pub use itertools_ as itertools;

#[cfg(feature = "lazy_static")] extern crate lazy_static as lazy_static_;
#[cfg(feature = "lazy_static")] pub use lazy_static_ as lazy_static;

#[cfg(feature = "libc")] extern crate libc as libc_;
#[cfg(feature = "libc")] pub use libc_ as libc;

#[cfg(feature = "log")] extern crate log as log_;
#[cfg(feature = "log")] pub use log_ as log;

#[cfg(feature = "num")] extern crate num as num_;
#[cfg(feature = "num")] pub use num_ as num;

#[cfg(feature = "rand")] extern crate rand as rand_;
#[cfg(feature = "rand")] pub use rand_ as rand;

#[cfg(feature = "regex")] extern crate regex as regex_;
#[cfg(feature = "regex")] pub use regex_ as regex;

#[cfg(feature = "rustc_serialize")] extern crate rustc_serialize as rustc_serialize_;
#[cfg(feature = "rustc_serialize")] pub use rustc_serialize_ as rustc_serialize;

#[cfg(feature = "semver")] extern crate semver as semver_;
#[cfg(feature = "semver")] pub use semver_ as semver;

#[cfg(feature = "tempdir")] extern crate tempdir as tempdir_;
#[cfg(feature = "tempdir")] pub use tempdir_ as tempdir;

#[cfg(feature = "time")] extern crate time as time_;
#[cfg(feature = "time")] pub use time_ as time;

#[cfg(feature = "toml")] extern crate toml as toml_;
#[cfg(feature = "toml")] pub use toml_ as toml;

#[cfg(feature = "url")] extern crate url as url_;
#[cfg(feature = "url")] pub use url_ as url;
