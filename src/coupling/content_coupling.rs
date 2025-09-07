//! ./content_coupling.rs
//!
//! One module directly accesses or modifies the private internal data of another
//!
//! Problem
#[derive(Default)]
pub struct Storage {
    view_count: usize,
}

impl Storage {
    /// Exposed API and being consumed by external module (in same package)
    /// Or External Crate
    /// What happens if we change name
    pub fn show_view_count(self) {
        println!("{} Reporting {}", "-".repeat(40), "-".repeat(40));
        println!("Total view count is :{}", self.view_count);
        println!("{} End of Reporting {}", "-".repeat(37), "-".repeat(37));
    }
}

impl Reportable for Storage {
    fn view_count(self) -> usize {
        self.view_count
    }
}

/// Solution
/// (D)ependency Inversion Principle in SOLID
pub trait Reportable {
    fn view_count(self) -> usize;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn external_library() {
        let storage: Storage = Storage::default();

        storage.show_view_count();

        assert!(false)
    }

    #[test]
    fn external_library_contract() {
        let storage: Storage = Storage::default();

        fn report_view_code<T>(rp: T)
        where
            T: Reportable,
        {
            println!("Total view count is : {}", rp.view_count());
        }

        report_view_code(storage);

        assert!(false)
    }
}
