use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// todo documentation
pub mod domain {
    /// todo documentation
    pub mod model {
        /// Represents the YAML configuration file provided by the user.
        pub struct Config {
            /// Collection of `String` representing the path to a git repository on disk.
            pub repositories: Vec<String>,
        }
    }
}

/// todo documentation
fn main() {
    // Create a path to the sample configuration file.
    let path = Path::new("/home/zoo/development/git/gawk.toml");
    let display = path.display();

    // Open the path in read-only mode.
    let mut config_file = match File::open(&path) {
        Ok(config_file) => config_file,
        Err(e) => panic!("Unable to open {}: {}", display, e.description()),
    };

    // Read file contents into a string
    let mut contents = String::new();
    match config_file.read_to_string(&mut contents) {
        Ok(_) => println!("{}", contents),
        Err(e) => panic!("Unable to read {}: {}", display, e.description()),
    }
}
