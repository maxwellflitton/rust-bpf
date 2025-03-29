#![no_std]
#![no_main]

use aya_ebpf::{
    bindings::xdp_action, 
    macros::{xdp, map}, 
    programs::XdpContext,
    maps::PerCpuArray,
};
use aya_log_ebpf::info;

use core::mem;
use network_types::{
    eth::{EthHdr, EtherType},
    ip::{IpProto, Ipv4Hdr},
    tcp::TcpHdr,
    udp::UdpHdr
};


const CPU_CORES: u32 = 16;


#[map(name="PKT_CNT_ARRAY")]
static mut PACKET_COUNTER: PerCpuArray<u32> = PerCpuArray::with_max_entries(CPU_CORES , 0);


#[inline(always)] // 
fn ptr_at<T>(ctx: &XdpContext, offset: usize) -> Result<*const T, ()> {
    let start = ctx.data();
    let end = ctx.data_end();
    let len = mem::size_of::<T>();

    if start + offset + len > end {
        return Err(());
    }

    Ok((start + offset) as *const T)
}

#[xdp]
pub fn packet_tracer(ctx: XdpContext) -> u32 {
    match try_xdp_firewall(ctx) {
        Ok(ret) => ret,
        Err(_) => xdp_action::XDP_ABORTED,
    }
}

fn try_xdp_firewall(ctx: XdpContext) -> Result<u32, ()> {
    // Parse Ethernet header
    let ethhdr = ptr_at::<EthHdr>(&ctx, 0)?;
    if unsafe { (*ethhdr).ether_type } != EtherType::Ipv4 {
        return Ok(xdp_action::XDP_PASS); // Only interested in IPv4
    }

    // Parse IPv4 header
    let ipv4hdr = ptr_at::<Ipv4Hdr>(&ctx, EthHdr::LEN)?;
    let source_addr = u32::from_be(unsafe { (*ipv4hdr).src_addr });

    // Parse TCP or UDP and get source port
    let source_port = match unsafe { (*ipv4hdr).proto } {
        IpProto::Tcp => {
            let tcphdr = ptr_at::<TcpHdr>(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)?;
            u16::from_be(unsafe { (*tcphdr).source })
            
        }
        IpProto::Udp => {
            let udphdr = ptr_at::<UdpHdr>(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)?;
            u16::from_be(unsafe { (*udphdr).source })
        }
        _ => return Err(()), // Abort unsupported protocols
    };

    let dest_port = match unsafe { (*ipv4hdr).proto } {
        IpProto::Tcp => {
            let tcphdr = ptr_at::<TcpHdr>(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)?;
            u16::from_be(unsafe { (*tcphdr).dest })
        }
        IpProto::Udp => {
            let udphdr = ptr_at::<UdpHdr>(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)?;
            u16::from_be(unsafe { (*udphdr).dest })
        }
        _ => return Err(()), // Abort unsupported protocols
    };

    if dest_port > 5000 && dest_port < 10000 {
        // Log IP and port
        info!(&ctx, "SRC IP: {:i}, SRC PORT: {}, DEST PORT: {}", source_addr, source_port, dest_port);

        // Update per-CPU packet counter
        unsafe {
            let container = PACKET_COUNTER.get_ptr_mut(0).ok_or(())?;
            *container += 1;
            info!(&ctx, "counter: {}", *container);
        }
    }

    // Log IP and port
    // info!(&ctx, "SRC IP: {:i}, SRC PORT: {}, DEST PORT: {}", source_addr, source_port, dest_port);

    // // Update per-CPU packet counter
    // unsafe {
    //     let container = PACKET_COUNTER.get_ptr_mut(0).ok_or(())?;
    //     *container += 1;
    //     info!(&ctx, "counter: {}", *container);
    // }

    Ok(xdp_action::XDP_PASS) // Let the packet go through
}


#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
