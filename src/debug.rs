use x86_64::{
    instructions::tables::sgdt,
    registers::{
        control::{Cr0, Cr2, Cr3},
        segmentation::{Segment, CS, DS, SS},
    },
};

use crate::println;

pub fn print_debug_info() {
    let cr0 = Cr0::read();
    let cr2 = Cr2::read();
    let cr3 = Cr3::read();
    let flags = x86_64::registers::rflags::read();
    println!(
        "cr0 = {:?}\ncr2 = {:?}\ncr3 = {:?}\nflags = {:?}",
        cr0, cr2, cr3, flags
    );

    let gdtp = sgdt();
    let gdt: [u64; 4] = unsafe { *gdtp.base.as_ptr() };
    println!("{:#016x?}", gdt);

    let cs = CS::get_reg();
    let ds = DS::get_reg();
    let ss = SS::get_reg();
    println!("cs = {:?}, ds = {:?}, ss = {:?}", cs, ds, ss);
}
