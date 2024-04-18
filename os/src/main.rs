#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;



#[macro_use]
mod lang_items;
mod sbi;
mod console;

global_asm!(include_str!("entry.asm"));

fn clear_bss() {
    extern "C" {
        fn ebss();
        fn sbss();
    }
    (sbss as usize..ebss as usize).for_each(|x| {
        unsafe {
            (x as *mut u8).write_volatile(0)
        }}
    );
}

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    panic!("shutdown");
}