use anyhow::Context as _;
use aya::programs::{Xdp, XdpFlags};
use clap::Parser;
#[rustfmt::skip]
use log::{debug, warn};
use tokio::signal;
use aya::maps::PerCpuArray;
use aya::maps::PerCpuValues;
// use tokio::time::{sleep, Duration};


#[derive(Debug, Parser)]
struct Opt {
    #[clap(short, long, default_value = "eth0")]
    iface: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = Opt::parse();

    env_logger::init();

    // Bump the memlock rlimit. This is needed for older kernels that don't use the
    // new memcg based accounting, see https://lwn.net/Articles/837122/
    let rlim = libc::rlimit {
        rlim_cur: libc::RLIM_INFINITY,
        rlim_max: libc::RLIM_INFINITY,
    };
    let ret = unsafe { libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlim) };
    if ret != 0 {
        debug!("remove limit on locked memory failed, ret is: {}", ret);
    }

    // This will include your eBPF object file as raw bytes at compile-time and load it at
    // runtime. This approach is recommended for most real-world use cases. If you would
    // like to specify the eBPF program at runtime rather than at compile-time, you can
    // reach for `Bpf::load_file` instead.
    let mut ebpf = aya::Ebpf::load(aya::include_bytes_aligned!(concat!(
        env!("OUT_DIR"),
        "/packet-tracer"
    )))?;
    if let Err(e) = aya_log::EbpfLogger::init(&mut ebpf) {
        // This can happen if you remove all log statements from your eBPF program.
        warn!("failed to initialize eBPF logger: {}", e);
    }
    let Opt { iface } = opt;
    let program: &mut Xdp = ebpf.program_mut("packet_tracer").unwrap().try_into()?;
    program.load()?;
    program.attach(&iface, XdpFlags::default())
        .context("failed to attach the XDP program with default flags - try changing XdpFlags::default() to XdpFlags::SKB_MODE")?;


    // let array = PerCpuArray::try_from("PKT_CNT_ARRAY").unwrap();
    let array = PerCpuArray::try_from(ebpf.map_mut("PKT_CNT_ARRAY").unwrap())?;
    let test = ebpf.map_mut("PKT_CNT_ARRAY");

    loop {
        let cc: PerCpuValues<u32> = array.get(&0, 0)?;
        let mut total : u32 =  0;
        for i in 1..16 {
            total += cc[i];
        }
        println!("\n\n\n\ntotal in user space: {}\n\n\n\n", total);
        // tokio::time::sleep(Duration::from_secs(1)).await;
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    // Ok(())
}
