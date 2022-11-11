#![no_std]
#![no_main]

use core::panic::PanicInfo;
use tetris::kernel;
use tetris::kernel::graphics::TTYFrame;
use tetris::kernel::interrupts;
use core::fmt::Write;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    kernel::init();
    
    let mut frame = TTYFrame::new();
    loop {
        let current_time = interrupts::current_time();
        let keyboard_state = interrupts::current_keyboard_state();

        tetris::game::run(current_time, keyboard_state, &mut frame);
        frame.flush();        
        frame = TTYFrame::new();

        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut frame = kernel::graphics::TTYFrame::new();
    core::write!(&mut frame, "{}", info).unwrap();    
    frame.flush();

    loop {
        x86_64::instructions::hlt();
    }
}


