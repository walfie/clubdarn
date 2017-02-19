use clap;
use clubdarn;
#[cfg(feature = "library")]
use id3;
use serde_json;

error_chain! {
    links {
        Client(clubdarn::Error, clubdarn::error::ErrorKind);
    }

    foreign_links {
        Json(serde_json::Error);
        Input(clap::Error);
        Id3(id3::Error);
    }
}

// If compiled without "library" feature, create a dummy Id3 Error
#[cfg(not(feature = "library"))]
mod id3 {
    use std;
    use std::fmt;

    #[derive(Debug)]
    pub struct Error;

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "")
        }
    }

    impl std::error::Error for Error {
        fn description(&self) -> &str {
            "Unused"
        }

        fn cause(&self) -> Option<&std::error::Error> {
            None
        }
    }
}
