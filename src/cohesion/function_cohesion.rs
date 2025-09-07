//! ./function_cohesion.rs
//!
//! The highest and most desirable form of cohesion,
//! where all elements within a module work together to achieve a single,
//! well-defined task
//!
//! Higest Form of Cohesion

mod circle_geometry {
    use std::f64::consts::PI;

    /// A struct to represent a circle with a radius.
    pub struct Circle {
        radius: f64,
    }

    impl Circle {
        pub fn new(radius: f64) -> Self {
            Self { radius }
        }

        // A function to calculate the area of the circle.
        pub fn calculate_area(&self) -> f64 {
            PI * self.radius * self.radius
        }

        // A function to calculate the circumference of the circle.
        pub fn calculate_circumference(&self) -> f64 {
            2.0 * PI * self.radius
        }
    }
}

#[cfg(test)]
mod test {
    use super::circle_geometry;

    #[test]
    fn circle_geometry_calculation() {
        use circle_geometry::Circle;

        let my_circle = Circle::new(5.0);
        let area = my_circle.calculate_area();
        let circumference = my_circle.calculate_circumference();

        println!("Circle with radius 5.0:");
        println!("Area: {}", area);
        println!("Circumference: {}", circumference);
        assert!(false);
    }
}
