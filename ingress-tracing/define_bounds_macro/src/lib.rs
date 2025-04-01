use proc_macro::TokenStream;
use quote::quote;

// #[proc_macro]
// pub fn define_upperbound_lowerbound(_item: TokenStream) -> TokenStream {
//     let lowerbound = std::env::var("LOWERBOUND_PORT").unwrap_or_else(|_| "5000".to_string());
//     let upperbound = std::env::var("UPPERBOUND_PORT").unwrap_or_else(|_| "10000".to_string());

//     let lowerbound_num = match lowerbound.parse::<u32>() {
//         Ok(num) => num,
//         Err(_) => {
//             panic!("env LOWERBOUND_PORT is {} and should be a u32", lowerbound);
//         }
//     };

//     let upperbound_num = match upperbound.parse::<u32>() {
//         Ok(num) => num,
//         Err(_) => {
//             panic!("env UPPERBOUND_PORT is {} and should be a u32", upperbound);
//         }
//     };
//     let port_range = upperbound_num - lowerbound_num;

//     let expanded = quote! {
//         pub const LOWERBOUND_PORT: u32 = #lowerbound_num;
//         pub const UPPERBOUND_PORT: u32 = #upperbound_num;
//         pub const PORT_RANGE: u32 = #port_range;
//     };

//     TokenStream::from(expanded)
// }

#[proc_macro]
pub fn define_upperbound_lowerbound(item: TokenStream) -> TokenStream {
    let args = item.to_string();
    let parts: Vec<&str> = args.split(',').map(|s| s.trim()).collect();
    if parts.len() != 2 {
        panic!("Expected two comma-separated integer values: lowerbound, upperbound");
    }

    let lowerbound_num: u32 = parts[0].parse().expect("Invalid LOWERBOUND_PORT");
    let upperbound_num: u32 = parts[1].parse().expect("Invalid UPPERBOUND_PORT");

    if lowerbound_num >= upperbound_num {
        panic!("LOWERBOUND_PORT must be less than UPPERBOUND_PORT");
    }

    let port_range = upperbound_num - lowerbound_num;

    let expanded = quote! {
        pub const LOWERBOUND_PORT: u32 = #lowerbound_num;
        pub const UPPERBOUND_PORT: u32 = #upperbound_num;
        pub const PORT_RANGE: u32 = #port_range;
    };

    TokenStream::from(expanded)
}

