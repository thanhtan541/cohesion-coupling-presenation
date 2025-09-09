//! Cohesion is a measure of the degree to which the elements of the module are functionally related.
//! It is the degree to which all elements directed towards performing a single task are contained in the component.
//! Basically, cohesion is the internal glue that keeps the module together.
//! A good software design will have high cohesion.

#[rustfmt::skip]
mod function_cohesion;
mod coincidental_cohesion;
mod communicational_cohesion;
mod logical_cohesion;
mod procedural_cohesion;
mod sequence_cohesion;
mod temporal_cohesion;

// mod function_cohesion;
// mod sequence_cohesion;
// mod communicational_cohesion;
// mod procedural_cohesion;
// mod temporal_cohesion;
// mod logical_cohesion;
// mod coincidental_cohesion;
//
