use rosu_mem::process::{Process, ProcessTraits};
use crate::reader::structs::State;

pub(crate) fn get_status(p: &Process, state: &mut State) -> u32 {
    let status_ptr = p.read_i32(state.addresses.status - 0x4).unwrap();
    p.read_u32(status_ptr).unwrap()
}
