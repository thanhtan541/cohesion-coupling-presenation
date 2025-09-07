//! ./control_coupling.rs
//!
//! One module influences the control flow of another module by passing specific data as an argument
//!
//! Problem

// Callee
mod formatter {
    // A public enum is used as a control flag.
    pub enum Format {
        PlainText,
        Html,
    }

    pub fn format_data(data: String, format: Format) -> String {
        match format {
            Format::PlainText => {
                // Formatting for plain text
                println!("Formatting as plain text.");
                data
            }
            Format::Html => {
                // Formatting for HTML
                println!("Formatting as HTML.");
                format!("<p>{}</p>", data)
            }
        }
    }
}

// Caller
mod report_generator {
    use super::formatter::{self, Format};

    pub fn generate_report(format_type: Format) -> String {
        let data = "Some report data".to_string();
        // The generator directly passes a control flag to the formatter.
        // It knows about the formatter's internal logic.
        formatter::format_data(data, format_type)
    }
}

#[cfg(test)]
mod tests {
    use super::{formatter::*, report_generator::*};

    #[test]
    fn caller_dictate_callee() {
        let plain_report = generate_report(Format::PlainText);
        println!("{}", plain_report);

        let html_report = generate_report(Format::Html);
        println!("{}", html_report);
        assert!(false);
    }
}

/// Solution
pub struct App;

mod traits {
    // Define a public, stable interface.
    pub trait Formatter {
        fn format(&self, data: &str) -> String;
    }
}

mod plain_text_formatter {
    use super::traits::Formatter;

    pub struct PlainTextFormatter;

    impl Formatter for PlainTextFormatter {
        fn format(&self, data: &str) -> String {
            println!("Formatting as plain text.");
            data.to_string()
        }
    }
}

mod html_formatter {
    use super::traits::Formatter;

    pub struct HtmlFormatter;

    impl Formatter for HtmlFormatter {
        fn format(&self, data: &str) -> String {
            println!("Formatting as HTML.");
            format!("<p>{}</p>", data)
        }
    }
}

mod report_generator_v2 {
    use super::traits::Formatter;

    // Now, the generator depends only on the stable `Formatter` trait.
    pub fn generate_report(formatter: &impl Formatter) -> String {
        let data = "Some report data";
        formatter.format(data)
    }
}
