//! ./common_coupling.rs
//!
//! Common coupling occurs when two or more modules share access to the same global, mutable state
//!
//! Problem

use std::sync::{Arc, Mutex};
#[cfg(test)]
mod tests {
    #[test]
    fn modify_shared_stated() {
        let state_one: i64 = 11;
        let state_two: i64 = 12;

        let state_one_ptr = &state_one as *const i64;
        let state_one_addr: usize = unsafe { std::mem::transmute(state_one_ptr) };
        println!(
            "state_one_ptr: {} ({:p}...0x{:x})",
            state_one,
            state_one_ptr,
            state_one_addr + 7
        );

        let state_two_ptr = &state_two as *const i64;
        let state_two_addr: usize = unsafe { std::mem::transmute(state_two_ptr) };
        println!(
            "state_two_ptr: {} ({:p}...0x{:x})",
            state_two,
            state_two_ptr,
            state_two_addr + 7
        );

        println!("Modifing the pointer value of state_one_ptr");
        unsafe {
            let new_addr = state_one_ptr.byte_offset(8);
            println!("{:p} -> {:p}", state_one_ptr, new_addr);
            let value_from_new_addr: i64 = *new_addr;
            println!("Value of new_addr: {}", value_from_new_addr);
        }

        // Imagine if two or more threads modifies these state in arbitrary order

        assert!(false);
    }
}

/// Solution
/// Use: Avoid Raw pointer
/// And use thread-safe structs if sharing state is needed
pub struct State {
    count: Arc<Mutex<i32>>,
}
