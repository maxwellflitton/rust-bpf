#![no_std]
#![no_main]

use core::mem;
use network_types::{
    eth::{EthHdr, EtherType},
    ip::{IpProto, Ipv4Hdr},
    tcp::TcpHdr,
    udp::UdpHdr
};
use aya_ebpf::{
    bindings::TC_ACT_PIPE, 
    macros::{map, socket_filter}, 
    maps::PerCpuArray, 
    programs::SkBuffContext
};
use aya_ebpf::bindings::iphdr;
use aya_ebpf::bindings::tcphdr;
use aya_log_ebpf::info;


const ETH_HDR_LEN: usize = mem::size_of::<EthHdr>();
const IP_HDR_LEN: usize = mem::size_of::<iphdr>();
const TCP_HDR_LEN: usize = mem::size_of::<tcphdr>();

#[repr(C)]
pub struct Buf {
   pub buf: [u8; 1500],
}

#[map]
pub static mut BUF: PerCpuArray<Buf> = PerCpuArray::with_max_entries(1, 0);

#[socket_filter]
pub fn socket_tracing(ctx: SkBuffContext) -> i64 {
    // let buf = unsafe {
    //     let ptr = BUF.get_ptr_mut(0).ok_or(TC_ACT_PIPE).unwrap();
    //     &mut *ptr
    // };
    // let offset = ETH_HDR_LEN + IP_HDR_LEN + TCP_HDR_LEN;
    // ctx.load_bytes(offset, &mut buf.buf).map_err(|_| TC_ACT_PIPE).unwrap();
    info!(&ctx, "got a packet from a socket");
    0
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
