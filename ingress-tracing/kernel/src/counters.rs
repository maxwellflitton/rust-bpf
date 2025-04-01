use aya_ebpf::{maps::PerCpuArray, macros::map};
use define_bounds_macro::define_upperbound_lowerbound;


// Macro is envoked here because we need to define the number at compile time.
define_upperbound_lowerbound!();


#[map(name="INGRESS_PORT_COUNTERS")]
pub static mut INGRESS_PORT_COUNTERS: PerCpuArray<u32> = PerCpuArray::with_max_entries(PORT_RANGE , 0);

#[map(name="EGRESS_PORT_COUNTERS")]
pub static mut EGRESS_PORT_COUNTERS: PerCpuArray<u32> = PerCpuArray::with_max_entries(PORT_RANGE , 0);



pub fn increment_ingress_port(port: u16) {
    let index = port as u32 - LOWERBOUND_PORT;
    unsafe {
        if let Some(counter) = INGRESS_PORT_COUNTERS.get_ptr_mut(index) {
            *counter += 1;
        }
    }
}


pub fn increment_egress_port(port: u16) {
    let index = port as u32 - LOWERBOUND_PORT;
    unsafe {
        if let Some(counter) = EGRESS_PORT_COUNTERS.get_ptr_mut(index) {
            *counter += 1;
        }
    }
}
