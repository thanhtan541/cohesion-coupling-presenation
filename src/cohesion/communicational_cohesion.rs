//! ./communicational_cohesion.rs
//!
//! A type of cohesion where elements within a module are grouped together
//! because they operate on the same central piece of data or data structure
//!
//! Good cohesion

mod customer {
    // The central data structure.
    pub struct CustomerProfile {
        first_name: String,
        last_name: String,
        email: String,
        address: String,
    }

    impl CustomerProfile {
        pub fn new(first_name: &str, last_name: &str, email: &str, address: &str) -> Self {
            CustomerProfile {
                first_name: first_name.to_string(),
                last_name: last_name.to_string(),
                email: email.to_string(),
                address: address.to_string(),
            }
        }

        // A function that reads from the data.
        pub fn get_full_name(&self) -> String {
            format!("{} {}", self.first_name, self.last_name)
        }

        // A function that writes to the data.
        pub fn update_email(&mut self, new_email: &str) {
            println!("Updating email for {}", self.get_full_name());
            self.email = new_email.to_string();
        }

        // Another function that reads from the data.
        pub fn get_mailing_label(&self) -> String {
            format!("{}\n{}", self.get_full_name(), self.address)
        }
    }
}

#[cfg(test)]
mod test {
    use super::customer;

    #[test]
    fn report_user_profile() {
        let mut profile = customer::CustomerProfile::new(
            "Alice",
            "Smith",
            "alice.smith@example.com",
            "123 Main Street",
        );

        println!("Full Name: {}", profile.get_full_name());
        println!("Mailing Label:\n{}", profile.get_mailing_label());

        profile.update_email("alice.s@newmail.com");
        println!("New email set, profile is updated.");
    }
}

/// Good but can be better
// A new, functionally cohesive module focused only on reporting.
mod reporting_module {
    use super::customer::CustomerProfile;

    pub fn generate_customer_summary(profile: &CustomerProfile) -> String {
        // Only does one thing: generates a summary.
        format!(
            "Summary for {}: Last email update was 2 days ago.",
            profile.get_full_name()
        )
    }
}
