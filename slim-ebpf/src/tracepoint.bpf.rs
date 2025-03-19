#![no_std]
#![no_main]

use aya_bpf::{macros::tracepoint, programs::TracePointContext};
use aya_log_ebpf::info;
#[tracepoint(name = "syscall_enter")]
pub fn syscall_enter(ctx: TracePointContext) -> u32 {
    match unsafe { try_syscall_enter(ctx) } {
        Ok(ret) => ret,
        Err(_) => 1,
    }
}
unsafe fn try_syscall_enter(ctx: TracePointContext) -> Result<u32, u32> {
    info!(&ctx, "System call entered");
    Ok(0)
}
