//! ./temporal_cohesion.rs
//!
//! When elements within a module are grouped together
//! because they are executed at the same time or within a specific,
//! limited time frame during a program's execution
//!
//! Problem

mod system {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::Write;

    // This function exhibits temporal cohesion.
    // It groups several unrelated initialization tasks together.
    pub fn startup() {
        println!("Starting up the application...");

        // 1. Initialize an in-memory cache.
        let mut cache: HashMap<String, String> = HashMap::new();
        cache.insert("status".to_string(), "running".to_string());
        println!("Cache initialized.");

        // 2. Create a log file.
        let mut file = File::create("app.log").expect("Failed to create log file");
        file.write_all(b"Application started.\n")
            .expect("Failed to write to log file");
        println!("Log file created.");

        // 3. Set a global configuration variable.
        // This relies on some external, global state pattern.
        // For demonstration, let's just print.
        println!("Configuration loaded from environment.");

        println!("Startup complete.");
    }
}

#[cfg(test)]
mod test {
    use super::system;

    #[test]
    fn init_system() {
        system::startup();
        assert!(false);
    }
}

/// Solution
mod system_v2 {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::Write;

    // A functionally cohesive function for initializing the cache.
    fn initialize_cache() -> HashMap<String, String> {
        let mut cache: HashMap<String, String> = HashMap::new();
        cache.insert("status".to_string(), "running".to_string());
        println!("Cache initialized.");
        cache
    }

    // A functionally cohesive function for setting up logging.
    fn setup_logging() {
        let mut file = File::create("app.log").expect("Failed to create log file");
        file.write_all(b"Application started.\n")
            .expect("Failed to write to log file");
        println!("Log file created.");
    }

    // A functionally cohesive function for loading configuration.
    fn load_config() {
        // ... Logic to load configuration ...
        println!("Configuration loaded from environment.");
    }

    // A "coordinator" function that orchestrates the startup process.
    pub fn startup() {
        println!("Starting up the application...");
        initialize_cache();
        setup_logging();
        load_config();
        println!("Startup complete.");
    }
}
