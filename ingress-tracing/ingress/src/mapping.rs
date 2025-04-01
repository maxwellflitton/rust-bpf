use kernel::counters::LOWERBOUND_PORT;
use aya::maps::PerCpuArray;
use aya::maps::PerCpuValues;
use aya::Ebpf;


pub fn get_and_wipe_ingress_count(port: u16, epbf: &mut Ebpf) -> Result<Option<u32>, String> {
    get_and_wipe_count(port, "EGRESS_PORT_COUNTERS", epbf)
}


pub fn get_and_wipe_egress_count(port: u16, epbf: &mut Ebpf) -> Result<Option<u32>, String> {
    get_and_wipe_count(port, "EGRESS_PORT_COUNTERS", epbf)
}

fn get_and_wipe_count(port: u16, map_name: &str, epbf: &mut Ebpf) -> Result<Option<u32>, String> {
    let index = port as u32 - LOWERBOUND_PORT;
    let map = match epbf.map_mut(map_name) {
        None => return Ok(None),
        Some(map) => map
    };
    let array = match PerCpuArray::try_from(map) {
        Ok(unwrapped_array) => unwrapped_array,
        Err(e) => return Err(e.to_string())
    };
    let count: PerCpuValues<u32> = match array.get(&index, 0) {
        Ok(unwrapped_count) => unwrapped_count,
        Err(e) => return Err(e.to_string())
    };
    let raw_count = count;
    println!("here is the raw count: {:?}", raw_count);

    Ok(Some(44))

}