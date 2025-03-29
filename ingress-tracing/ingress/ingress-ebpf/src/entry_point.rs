use aya_ebpf::{
    programs::TcContext,
    maps::PerCpuArray,
};
use aya_log_ebpf::info;
use crate::network_packet::NetworkPacket;


// #define TC_ACT_SHOT 2  // drop
// #define TC_ACT_OK   0  // *not* drop, but NO redirection or continuation
// #define TC_ACT_PIPE 3  // let it go through the pipeline


pub fn ingress_entry_point(ctx: TcContext, map: &mut PerCpuArray<u32>) -> Result<u32, ()> {
    let packet = match NetworkPacket::from_context(&ctx) {
        Ok(outcome) => outcome,
        Err(code) => return Ok(code)
    };

    if packet.dest_port > 5000 && packet.dest_port < 10000 {
        // Log IP and port
        info!(
            &ctx, 
            "SRC IP: {:i}, SRC PORT: {}, DEST PORT: {}", 
            packet.source_addr, 
            packet.source_port, 
            packet.dest_port
        );

        // Update per-CPU packet counter
        unsafe {
            let container = map.get_ptr_mut(0).ok_or(())?;
            *container += 1;
            info!(&ctx, "counter: {}", *container);
        }
    }

    Ok(3) // Let the packet go through
}