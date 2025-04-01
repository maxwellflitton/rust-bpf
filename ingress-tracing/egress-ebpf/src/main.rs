#![no_std]
#![no_main]

mod entry_point;
mod filters;
use aya_ebpf::{
    macros::classifier, 
    programs::TcContext,
};

// #define TC_ACT_SHOT 2  // drop
// #define TC_ACT_OK   0  // *not* drop, but NO redirection or continuation
// #define TC_ACT_PIPE 3  // let it go through the pipeline

#[classifier]
pub fn ingress(ctx: TcContext) -> i32 {
    match entry_point::egress_entry_point(ctx) {
        Ok(ret) => ret.try_into().unwrap(),
        Err(_) => 1,
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
