use aya_ebpf::{maps::PerCpuArray, macros::map};
// use define_bounds_macro::define_upperbound_lowerbound;
pub use crate::generated_ports::{LOWERBOUND_PORT, UPPERBOUND_PORT, PORT_RANGE};


// Macro is envoked here because we need to define the number at compile time.
// define_upperbound_lowerbound!();


#[map(name="INGRESS_PORT_COUNTERS")]
pub static mut INGRESS_PORT_COUNTERS: PerCpuArray<u32> = PerCpuArray::with_max_entries(PORT_RANGE , 0);

#[map(name="EGRESS_PORT_COUNTERS")]
pub static mut EGRESS_PORT_COUNTERS: PerCpuArray<u32> = PerCpuArray::with_max_entries(PORT_RANGE , 0);



pub fn increment_ingress_port(port: u16) -> Result<(), u8> {
    let index = process_index(port)?;
    unsafe {
        if let Some(counter) = INGRESS_PORT_COUNTERS.get_ptr_mut(index) {
            *counter += 1;
        }
    }
    Ok(())
}


pub fn increment_egress_port(port: u16) -> Result<(), u8> {
    let index = process_index(port)?;
    unsafe {
        if let Some(counter) = EGRESS_PORT_COUNTERS.get_ptr_mut(index) {
            *counter += 1;
        }
    }
    Ok(())
}


fn process_index(port: u16) -> Result<u32, u8> {
    if LOWERBOUND_PORT > port as u32 {
        return Err(1)
    }
    let index = port as u32 - LOWERBOUND_PORT;
    if index > UPPERBOUND_PORT {
        return Err(2)
    }
    Ok(index)
}
