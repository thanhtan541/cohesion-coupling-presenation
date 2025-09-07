//! ./coincidental_cohesion.rs
//!
//! Occurs when a module's elements are grouped together because they perform similar,
//! but not identical, functions within the same logical category
//!
//! Problem

mod logging {
    // An enum is used as a control flag to determine the logging destination.
    pub enum LogDestination {
        Console,
        File,
        Database,
    }

    // This function exhibits logical cohesion.
    // It handles different logging tasks based on a control flag.
    // Temping to group multiple functions into one
    pub fn log_message(message: &str, destination: LogDestination) {
        match destination {
            LogDestination::Console => {
                println!("LOG (Console): {}", message);
            }
            LogDestination::File => {
                // In a real app, this would write to a file.
                println!("LOG (File): {}", message);
            }
            LogDestination::Database => {
                // In a real app, this would write to a database.
                println!("LOG (Database): {}", message);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::logging::{LogDestination, log_message};

    #[test]
    fn logging_to_different_io() {
        // The calling code dictates the specific action to take.
        log_message("System started.", LogDestination::Console);
        log_message("User logged in.", LogDestination::File);
        log_message(
            "Error: Database connection failed.",
            LogDestination::Database,
        );
    }
}

///Solution
mod loggers {
    // A trait defines a single responsibility: logging a message.
    pub trait Logger {
        fn log(&self, message: &str);
    }

    pub struct ConsoleLogger;
    impl Logger for ConsoleLogger {
        fn log(&self, message: &str) {
            println!("LOG (Console): {}", message);
        }
    }

    pub struct FileLogger;
    impl Logger for FileLogger {
        fn log(&self, message: &str) {
            println!("LOG (File): {}", message);
        }
    }

    // Any new logger type can implement this trait without changing existing code.
    pub struct DatabaseLogger;
    impl Logger for DatabaseLogger {
        fn log(&self, message: &str) {
            println!("LOG (Database): {}", message);
        }
    }
}
