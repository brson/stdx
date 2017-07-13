extern crate semver;

use semver::Version;

fn main() {
    // Construct Version objects
    assert!(Version::parse("1.2.3") == Ok(Version {
        major: 1,
        minor: 2,
        patch: 3,
        pre: vec!(),
        build: vec!(),
    }));

    // Compare Versions
    assert!(Version::parse("1.2.3-alpha") != Version::parse("1.2.3-beta"));
    assert!(Version::parse("1.2.3-alpha2") >  Version::parse("1.2.0"));

    // Increment patch number of mutable Version
    let mut bugfix_release = Version::parse("1.0.0").unwrap();
    bugfix_release.increment_patch();

    assert_eq!(Ok(bugfix_release), Version::parse("1.0.1"));
}