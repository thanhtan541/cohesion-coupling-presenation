//! ./coupling.rs
//!
//! This module introduces 6 types of coupling and organize the code from low to high.
//! The low coupling will be laid at the top and go down

#[rustfmt::skip]
mod stamp_coupling;
mod common_coupling; // ..
mod content_coupling;
mod control_coupling; // ..
mod external_coupling; // .. // Low
