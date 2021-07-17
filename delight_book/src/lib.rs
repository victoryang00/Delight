#![no_std]
#![no_main] 
#![feature(custom_test_frameworks,lang_items,start)]
#![test_runner(crate::test_runner)]
extern crate libc;

pub mod chapter1;
pub mod chapter2;
pub mod chapter3;

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    for test in tests {
        test();
    }
}

#[start]
pub extern "C" fn _start() -> ! {
    loop {}
}


#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

