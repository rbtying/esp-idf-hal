#![no_std]

pub mod delay;
pub mod errors;
pub mod gpio;
pub mod i2c;
pub mod serial;

#[cfg(feature = "alloc")]
pub mod rmt;

#[cfg(feature = "install_panic_handler")]
use core::panic::PanicInfo;

#[cfg(feature = "install_panic_handler")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        esp_idf_sys::abort();
        core::hint::unreachable_unchecked();
    }
}
