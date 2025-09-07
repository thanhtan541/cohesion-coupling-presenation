//! ./sequence_cohesion.rs
//!
//! A type of cohesion where the elements within a module are grouped
//! because the output from one element becomes the input for
//! the next element in a data processing pipeline
//!
//! Better than communicational cohesion

mod text_processor {
    // A private function that takes a string slice and trims whitespace.
    fn trim_whitespace(text: &str) -> String {
        println!("1. Trimming whitespace...");
        text.trim().to_string()
    }

    // A private function that takes a string and converts it to lowercase.
    // The output of `trim_whitespace` becomes the input for this function.
    fn to_lowercase(text: String) -> String {
        println!("2. Converting to lowercase...");
        text.to_lowercase()
    }

    // A private function that takes a string and replaces all spaces with hyphens.
    // The output of `to_lowercase` becomes the input for this function.
    fn replace_spaces_with_hyphens(text: String) -> String {
        println!("3. Replacing spaces with hyphens...");
        text.replace(' ', "-")
    }

    // The public function that orchestrates the entire sequential process.
    pub fn process_text(input: &str) -> String {
        let trimmed = trim_whitespace(input);
        let lowercased = to_lowercase(trimmed);
        replace_spaces_with_hyphens(lowercased)
    }
}

#[cfg(test)]
mod test {
    use super::text_processor;

    #[test]
    fn text_processor() {
        let raw_text = "   Hello World from Rust!   ";
        let processed_text = text_processor::process_text(raw_text);

        println!("Original: '{}'", raw_text);
        println!("Processed: '{}'", processed_text);
        assert!(false);
    }
}
