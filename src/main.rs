#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod vga_buffer;
mod interrupt_table;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

static CESIDOS: &str = "CesidOS";
static HELLO: &str = "Hello world!";
static SCROLLING_TEXT: [&str;5] = ["I", "Now", "Have", "Scrolling", "Text"];

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // println!("{}\n{}", CESIDOS, HELLO);
    for i in 1.. {
        for i in 0..1000000 {} // Yes I can't sleep
        if i > 23 && i < 24 + SCROLLING_TEXT.len() {
            println!("{}", SCROLLING_TEXT[i - 24]);
        } else {
            for _ in 0..i {
                print!("{}", i % 10);
            }
            println!();
        }
    }
    loop {}
}
