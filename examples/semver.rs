extern crate semver;

use semver::Version;

fn main() {
    assert!(Version::parse("1.0.0-alpha") < Version::parse("1.0.0-alpha.20170707"));
    assert!(Version::parse("1.0.0-alpha.20170707") < Version::parse("1.0.0-alpha.20170714"));
    assert!(Version::parse("1.0.0-alpha.20170712") < Version::parse("1.0.0-alpha.20170712.1"));
    assert!(Version::parse("1.0.0-alpha.20170712.1") < Version::parse("1.0.0-beta"));
    assert!(Version::parse("1.0.0-beta") < Version::parse("1.0.0-pre"));
    assert!(Version::parse("1.0.0-pre") < Version::parse("1.0.0-rc"));
    assert!(Version::parse("1.0.0-rc") < Version::parse("1.0.0"));

    let mut version = Version::parse("1.0.0").unwrap();
    version.increment_patch();
    assert_eq!(version, Version::parse("1.0.1").unwrap());
    version.increment_minor();
    assert_eq!(version, Version::parse("1.1.0").unwrap());
    version.increment_major();
    assert_eq!(version, Version::parse("2.0.0").unwrap());
}
