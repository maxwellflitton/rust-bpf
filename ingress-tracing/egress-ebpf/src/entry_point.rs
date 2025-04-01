use aya_ebpf::programs::TcContext;
use aya_log_ebpf::info;
use kernel::network_packet::NetworkPacket;
use kernel::counters::increment_egress_port;



// #define TC_ACT_SHOT 2  // drop
// #define TC_ACT_OK   0  // *not* drop, but NO redirection or continuation
// #define TC_ACT_PIPE 3  // let it go through the pipeline


// pub fn ingress_entry_point(ctx: TcContext, map: &mut PerCpuArray<u32>) -> Result<u32, ()> {
pub fn egress_entry_point(ctx: TcContext) -> Result<u32, ()> {
    let packet = match NetworkPacket::from_context(&ctx) {
        Ok(outcome) => outcome,
        Err(_) => return Ok(NetworkPacket::pass_through_return_status())
    };

    // TODO => apply a filter here if needed
    increment_egress_port(packet.dest_port);

    // Log IP and port
    // info!(
    //     &ctx, 
    //     "SRC IP: {:i}, SRC PORT: {}, DEST PORT: {}", 
    //     packet.source_addr, 
    //     packet.source_port, 
    //     packet.dest_port
    // );
    Ok(NetworkPacket::pass_through_return_status())
}