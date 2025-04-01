use aya::{programs::{tc, SchedClassifier, TcAttachType}, Ebpf, Pod};
use clap::Parser;
#[rustfmt::skip]
use log::{debug, warn};
use tokio::signal;
pub mod mapping;

#[derive(Debug, Parser)]
struct Opt {
    #[clap(short, long, default_value = "eth0")]
    iface: String,
}

enum ProgramType {
    Ingress,
    Egress
}

impl ProgramType {
    pub fn to_attachment_type(&self) -> TcAttachType {
        match self {
            ProgramType::Egress => TcAttachType::Egress,
            ProgramType::Ingress => TcAttachType::Ingress
        }
    }
}

fn load_program(opt: &Opt, program_type: ProgramType, mut ebpf: Ebpf) -> anyhow::Result<Ebpf> {
    if let Err(e) = aya_log::EbpfLogger::init(&mut ebpf) {
        // This can happen if you remove all log statements from your eBPF program.
        warn!("failed to initialize eBPF logger: {}", e);
    }


    let Opt { iface } = opt;
    // error adding clsact to the interface if it is already added is harmless
    // the full cleanup can be done with 'sudo tc qdisc del dev eth0 clsact'.
    let _ = tc::qdisc_add_clsact(&iface);
    let ingress_program: &mut SchedClassifier = ebpf.program_mut("ingress").unwrap().try_into()?;
    ingress_program.load()?;
    ingress_program.attach(&iface, program_type.to_attachment_type())?;  // for egress use TcAttachType::Egress 
    Ok(ebpf)
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

    // if dropped the programs will cease to run in kernel space
    let ingress = aya::Ebpf::load(aya::include_bytes_aligned!(concat!(
        env!("OUT_DIR"),
        "/ingress"
    )))?;
    let egress = aya::Ebpf::load(aya::include_bytes_aligned!(concat!(
        env!("OUT_DIR"),
        "/egress"
    )))?;
    let _egress = load_program(&opt, ProgramType::Egress, egress)?;
    let _ingress = load_program(&opt, ProgramType::Ingress, ingress)?;

    let ctrl_c = signal::ctrl_c();
    println!("Waiting for Ctrl-C...");
    ctrl_c.await?;
    println!("Exiting...");

    Ok(())
}
