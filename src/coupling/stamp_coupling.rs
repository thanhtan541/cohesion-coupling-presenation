//! ./stamp_coupling.rs
//!
//! Occurs when a function receives a complex data structure (like a struct or object)
//! as an argument but only uses a few of the fields within it
//!
//! Problem

mod telemetry {
    // A composite data structure with multiple fields.
    pub struct Record {
        pub id: u64,
        pub name: String,
        pub email: String,
        pub address: String,
    }
}

mod notification {
    use super::telemetry::Record;

    // This function exhibits stamp coupling.
    // It accepts the entire CustomerProfile struct but only needs the `name` field.
    pub fn send_telemetry(record: &Record) {
        println!("Sending record with {} id", record.id);
        // The function is coupled to the 'id' field, but carries along the 'id', 'email', and 'address' unnecessarily.
    }
}

#[cfg(test)]
mod test {
    use super::{notification::send_telemetry, telemetry::Record};

    #[test]
    fn sending_telemry() {
        let record = Record {
            id: 1,
            name: "John".to_string(),
            email: "john@mail.com".to_string(),
            address: "US".to_string(),
        };

        send_telemetry(&record);
    }
}

/// Solution
mod telemetry_v2 {
    // The struct can remain the same.
    pub struct Record {
        pub id: u64,
        pub name: String,
        pub email: String,
        pub address: String,
    }
}

// Module now takes only the specific data it needs.
mod notification_v2 {
    // The function is now decoupled from the internal structure of CustomerProfile.
    pub fn send_telemetry(log_ids: [&str]) {
        println!("Sending record with {} id", log_id);
    }
}
