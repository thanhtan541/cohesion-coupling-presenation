//! ./data_coupling.rs
//!
//! The best form of coupling, as it is the loosest and most resilient to change.
//! It occurs when modules interact by passing only primitive data types, such as integers, floats, or strings, through function parameters
//! In this type of coupling, modules are independent and do not rely on each other's internal structure or implementation details.
//!
//! Good coupling!

mod shop {
    // A composite data structure.
    pub struct Product {
        pub id: u64,
        pub name: String,
        pub price: f64,
    }
}

mod display {
    // This function exhibits data coupling.
    // It accepts only the primitive f64 price and not the entire Product struct.
    pub fn format_price(price: f64) -> String {
        format!("${:.2}", price)
    }
}

#[cfg(test)]
mod test {

    use super::{display::format_price, shop::Product};

    #[test]
    fn dislay_product() {
        let laptop = Product {
            id: 1,
            name: "Macbook".to_string(),
            price: 3.999,
        };

        let formatted_price = format_price(laptop.price);
        println!("The price of the {} is {}", laptop.name, formatted_price);
    }
}
