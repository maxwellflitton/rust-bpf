use std::env;
use std::fs;

fn main() {
    let lower = env::var("LOWERBOUND_PORT").unwrap_or_else(|_| "5000".to_string());
    let upper = env::var("UPPERBOUND_PORT").unwrap_or_else(|_| "10000".to_string());

    let content = format!("use define_bounds_macro::define_upperbound_lowerbound;\ndefine_upperbound_lowerbound!({}, {});", lower, upper);
    fs::write("src/generated_ports.rs", content).unwrap();
}
