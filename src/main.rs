#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

use vga_buffer::Writer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static CESIDOS: &str = "CesidOS";
static HELLO: &str = "Hello world!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut writer = Writer::new();
    writer.write_str(CESIDOS);
    writer.write_byte(b'\n');
    writer.write_str(HELLO);
    loop {}
}
