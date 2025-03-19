use aya::programs::TracePoint;
use aya::{Bpf, BpfLoader};
use std::convert::TryInto;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Load the compiled eBPF program
    let mut bpf = Bpf::load_file("target/bpf/release/tracepoint")?;
    
    // Attach the tracepoint to the syscall_enter event
    let program: &mut TracePoint = bpf.program_mut("syscall_enter").unwrap().try_into()?;
    program.load()?;
    program.attach("syscalls", "sys_enter")?;
    println!("eBPF program attached. Press Ctrl+C to exit.");
    // Wait for the user to terminate the program
    signal::ctrl_c().await?;
    Ok(())
}