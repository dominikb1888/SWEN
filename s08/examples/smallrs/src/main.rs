// fn main() {
//     std::process::exit(42)
// }

#![no_std]
#![no_main]

extern crate libc;

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
	// Similar to previous version, but unneccessary:
	// unsafe { libc::exit(42) }

    42
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
