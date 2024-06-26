use core::panic::PanicInfo;
use crate::{println, sbi::shutdown};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "Panicked at {}:{}, {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!(
            "Panicked at {}",
            info.message().unwrap()
        );
    }
    shutdown(true)
}