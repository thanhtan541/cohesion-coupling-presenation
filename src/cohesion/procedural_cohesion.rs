//! ./procedural_cohesion.rs
//!
//! When elements of a module are grouped together
//! because they must be executed in a specific sequence to accomplish a specific, broader task
//!
//! Problem

mod report_generation {
    // A step to retrieve raw data.
    fn fetch_raw_data() -> Vec<String> {
        println!("1. Fetching raw data from a source...");
        vec![
            "Data for section 1".to_string(),
            "Data for section 2".to_string(),
        ]
    }

    // A step to format the data for a cover page.
    fn format_cover_page() -> String {
        println!("2. Formatting the cover page...");
        "** Confidential Report **\n".to_string()
    }

    // A step to produce a summary of the data.
    fn summarize_data(data: &[String]) -> String {
        println!("3. Summarizing the data...");
        format!("Summary: {} items processed.", data.len())
    }

    // This public function exhibits procedural cohesion.
    // It groups a sequence of steps that must be executed in a specific order.
    pub fn generate_full_report() -> String {
        let raw_data = fetch_raw_data();
        let cover = format_cover_page();
        let summary = summarize_data(&raw_data);

        format!("{}\n---\n{}\n---\n{}", cover, summary, raw_data.join("\n"))
    }
}

#[cfg(test)]
mod test {
    use super::report_generation;

    #[test]
    fn generating_report() {
        let report = report_generation::generate_full_report();
        println!("\n--- Generated Report ---\n{}", report);

        assert!(false);
    }
}

/// Solution
mod report_parts {
    // A functionally cohesive function that only fetches data.
    pub fn fetch_raw_data() -> Vec<String> {
        println!("Fetching raw data...");
        vec![
            "Data for section 1".to_string(),
            "Data for section 2".to_string(),
        ]
    }

    // A functionally cohesive function that only formats cover pages.
    pub fn format_cover_page() -> String {
        println!("Formatting the cover page...");
        "** Confidential Report **\n".to_string()
    }

    // A functionally cohesive function that only summarizes data.
    pub fn summarize_data(data: &[String]) -> String {
        println!("Summarizing the data...");
        format!("Summary: {} items processed.", data.len())
    }

    // A functionally cohesive function to assemble the final report.
    pub fn assemble_report(cover: String, summary: String, data: &[String]) -> String {
        println!("Assembling the report...");
        format!("{}\n---\n{}\n---\n{}", cover, summary, data.join("\n"))
    }
}

fn main() {
    use report_parts::{assemble_report, fetch_raw_data, format_cover_page, summarize_data};

    // The functions are called in a specific sequence, but are more flexible.
    let raw_data = fetch_raw_data();
    let cover = format_cover_page();
    let summary = summarize_data(&raw_data);
    let report = assemble_report(cover, summary, &raw_data);

    println!("\n--- Generated Report ---\n{}", report);
}
