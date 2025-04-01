use kernel::counters::LOWERBOUND_PORT;
use aya::maps::PerCpuArray;
use aya::maps::PerCpuValues;
use aya::Ebpf;


pub fn get_and_wipe_ingress_count(port: u16, epbf: &mut Ebpf) -> Option<u32> {
    let index = port as u32 - LOWERBOUND_PORT;
    unsafe {
        let array = PerCpuArray::try_from(ebpf.map_mut("PKT_CNT_ARRAY").unwrap())?;
        if let Some(counter) = INGRESS_PORT_COUNTERS.get_ptr_mut(index) {
            let placeholder = *counter;
            *counter = 0;
            Some(placeholder)
        }
        else {
            None
        }
    }
}


pub fn get_and_wipe_egress_count(port: u16, epbf: &mut Ebpf) -> Option<u32> {
    let index = port as u32 - LOWERBOUND_PORT;
    unsafe {
        if let Some(counter) = EGRESS_PORT_COUNTERS.get_ptr_mut(index) {
            let placeholder = *counter;
            *counter = 0;
            Some(placeholder)
        }
        else {
            None
        }
    }
}

fn get_and_wipe_count(port: u16, map_name: &str, ebpf: &mut Ebpf) -> Result<Option<u32>, String> {
    let index = port as u32 - LOWERBOUND_PORT;
    let map = match ebpf.map_mut(map_name) {
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