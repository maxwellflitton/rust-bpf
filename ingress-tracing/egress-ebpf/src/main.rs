#![no_std]
#![no_main]

mod entry_point;
mod filters;
use aya_ebpf::{
    macros::{classifier, map}, 
    programs::TcContext,
    maps::PerCpuArray,
};

const CPU_CORES: u32 = 16;

#[map(name="PKT_CNT_ARRAY")]
static mut PACKET_COUNTER: PerCpuArray<u32> = PerCpuArray::with_max_entries(CPU_CORES , 0);

// #define TC_ACT_SHOT 2  // drop
// #define TC_ACT_OK   0  // *not* drop, but NO redirection or continuation
// #define TC_ACT_PIPE 3  // let it go through the pipeline

#[classifier]
pub fn ingress(ctx: TcContext) -> i32 {
    unsafe {
        match entry_point::ingress_entry_point(ctx, &mut PACKET_COUNTER) {
            Ok(ret) => ret.try_into().unwrap(),
            Err(_) => 1,
        }
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
