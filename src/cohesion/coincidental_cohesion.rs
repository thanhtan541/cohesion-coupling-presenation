//! ./coincidental_cohesion.rs
//!
//! The worst and lowest form of cohesion,
//! where the elements within a module are grouped together arbitrarily,
//! with no meaningful relationship between them.
//!
//! Problem

// This module groups unrelated functions.
mod utils {
    // Calculates the cosine of a number.
    pub fn calculate_cosine(angle: f64) -> f64 {
        angle.cos()
    }

    // Prints a greeting message.
    pub fn greet(name: &str) {
        println!("Hello, {}!", name);
    }

    // Parses a hex string into a u32.
    pub fn parse_hex_to_u32(hex_string: &str) -> Option<u32> {
        u32::from_str_radix(hex_string, 16).ok()
    }
}

/// Solution
mod math_utils {
    // This module is functionally cohesive, with a single purpose.
    pub fn calculate_cosine(angle: f64) -> f64 {
        angle.cos()
    }
}
