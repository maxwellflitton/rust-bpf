use aya_ebpf::{
    programs::TcContext,
    maps::PerCpuArray,
};
use aya_log_ebpf::info;
use kernel::network_packet::NetworkPacket;


// #define TC_ACT_SHOT 2  // drop
// #define TC_ACT_OK   0  // *not* drop, but NO redirection or continuation
// #define TC_ACT_PIPE 3  // let it go through the pipeline

const RETURN_CODE: u32 = 3;


pub fn ingress_entry_point(ctx: TcContext, map: &mut PerCpuArray<u32>) -> Result<u32, ()> {
    let packet = match NetworkPacket::from_context(&ctx) {
        Ok(outcome) => outcome,
        Err(_) => return Ok(RETURN_CODE)
    };

    // Log IP and port
    info!(
        &ctx, 
        "SRC IP: {:i}, SRC PORT: {}, DEST PORT: {}", 
        packet.source_addr, 
        packet.source_port, 
        packet.dest_port
    );
    Ok(RETURN_CODE) // Let the packet go through
}