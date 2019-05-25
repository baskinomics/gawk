/// todo documentation
mod domain {
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
    // Neato
    let config = domain::model::Config {
        repositories: vec!["test/one".to_string(), "test/two".to_string()],
    };

    // Noob
    let repos: String = config.repositories.into_iter().collect();
    println!("Repositories: {}", repos);
}
