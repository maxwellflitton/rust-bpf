use aya_ebpf::programs::TcContext;
use core::mem;
use network_types::{
    eth::{EthHdr, EtherType},
    ip::{IpProto, Ipv4Hdr},
    tcp::TcpHdr,
    udp::UdpHdr
};


#[inline(always)] // 
fn ptr_at<T>(ctx: &TcContext, offset: usize) -> Result<*const T, ()> {
    let start = ctx.data();
    let end = ctx.data_end();
    let len = mem::size_of::<T>();

    if start + offset + len > end {
        return Err(());
    }

    Ok((start + offset) as *const T)
}

pub struct NetworkPacket {
    pub source_port: u16,
    pub dest_port: u16,
    pub source_addr: u32
}

impl NetworkPacket {

    pub fn from_context(ctx: &TcContext) -> Result<Self, u32> {
        let ethhdr = ptr_at::<EthHdr>(&ctx, 0).map_err(|_| 2 as u32)?;
        if unsafe { (*ethhdr).ether_type } != EtherType::Ipv4 {
            return Err(2); // Only interested in IPv4
        }
        // Parse IPv4 header
        let ipv4hdr = ptr_at::<Ipv4Hdr>(&ctx, EthHdr::LEN).map_err(|_| 2 as u32)?;
        let source_addr = u32::from_be(unsafe { (*ipv4hdr).src_addr });

        // Parse TCP or UDP and get source port
        let source_port: u16;
        let dest_port: u16;
        match unsafe { (*ipv4hdr).proto } {
            IpProto::Tcp => {
                let tcphdr = ptr_at::<TcpHdr>(&ctx, EthHdr::LEN + Ipv4Hdr::LEN).map_err(|_| 2 as u32)?;
                source_port = u16::from_be(unsafe { (*tcphdr).source });
                dest_port = u16::from_be(unsafe { (*tcphdr).dest });
                
            }
            IpProto::Udp => {
                let udphdr = ptr_at::<UdpHdr>(&ctx, EthHdr::LEN + Ipv4Hdr::LEN).map_err(|_| 2 as u32)?;
                source_port = u16::from_be(unsafe { (*udphdr).source });
                dest_port = u16::from_be(unsafe { (*udphdr).dest });
            }
            _ => return Err(2), // Abort unsupported protocols
        };
        Ok(NetworkPacket { source_port, dest_port, source_addr })
    }

}


#[cfg(test)]
mod tests {

    #[test]
    fn test_constructor() {

    }

}