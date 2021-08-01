#![no_std]
#![no_main]

mod modules;


#[no_mangle]
pub extern "C" fn _start() -> ! {
	println!("Woah, the println works!");
	panic!("Yikes, this is an error.");
    loop {}
}