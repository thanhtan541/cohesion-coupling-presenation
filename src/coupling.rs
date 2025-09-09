//! ./coupling.rs
//!
//! Coupling is the measure of the degree of interdependence between the modules.
//! A good software will have low coupling.
//!

#[rustfmt::skip]
mod data_coupling; // High
mod common_coupling;
mod content_coupling;
mod control_coupling;
mod external_coupling;
mod stamp_coupling; // Low
//
// mod data_coupling; // High
// mod stamp_coupling;
// mod control_coupling;
// mod external_coupling;
// mod common_coupling;
// mod content_coupling; // Low
