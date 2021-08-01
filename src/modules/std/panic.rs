use core::panic::PanicInfo;

use crate::println;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { // This is the function called when the kernel panics
	println!("{}", _info);
	loop {}
}
