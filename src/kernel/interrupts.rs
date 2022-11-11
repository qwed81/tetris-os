use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptStackFrame, InterruptDescriptorTable};
use pic8259::ChainedPics;
use spin;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt[InterruptIndex::Timer as usize].set_handler_fn(timer_handler);
        idt[InterruptIndex::Keyboard as usize].set_handler_fn(keyboard_handler);
        idt
    };
}

pub fn init() {
    IDT.load();
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

// software interrupts

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame,
    _error_code: u64) -> !
{
    panic!("double fault occured: {:?}", stack_frame);
}

// hardware interrupts
pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> = spin::Mutex::new(unsafe {
    ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)
});

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard
}

static mut CURRENT_TIME: u64 = 0;

// increment the CURRENT_TIME variable
extern "x86-interrupt" fn timer_handler(_stack_frame: InterruptStackFrame) {
    unsafe { CURRENT_TIME += 1; }

    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer as u8);
    }
}

pub fn current_time() -> u64 {
    // because its read/only, it will always give back some
    // valid time, even though it might not be exact, that is OK
    unsafe { CURRENT_TIME }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Key {
    UpArrow = 0,
    DownArrow,
    LeftArrow,
    RightArrow,
    Space
}

#[derive(Clone)]
pub struct KeyboardState {
    key_down: [bool; 5],
    input_version: u64
}

impl KeyboardState {
    pub fn blank() -> KeyboardState {
        KeyboardState {
            key_down: [false; 5],
            input_version: 0
        }
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        self.key_down[key as usize]
    }

    pub fn input_version(&self) -> u64 {
        self.input_version
    }
}

static mut KEYBOARD_STATE: KeyboardState = KeyboardState {
    key_down: [false; 5],
    input_version: 0
};

extern "x86-interrupt" fn keyboard_handler(_stack_frame: InterruptStackFrame) {
    use x86_64::instructions::port::Port;

    let mut port = Port::new(0x60);
    let scan_code: u8 = unsafe { port.read() };
    process_scan_code(scan_code);

    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard as u8);
    }
}

fn process_scan_code(scan_code: u8) {
    unsafe {
        KEYBOARD_STATE.key_down = [false; 5];       
        match scan_code {
            72 => KEYBOARD_STATE.key_down[Key::UpArrow as usize] = true,
            200 => KEYBOARD_STATE.key_down[Key::UpArrow as usize] = false,
            80 => KEYBOARD_STATE.key_down[Key::DownArrow as usize] = true,
            208 => KEYBOARD_STATE.key_down[Key::DownArrow as usize] = false,
            77 => KEYBOARD_STATE.key_down[Key::RightArrow as usize] = true,
            205 => KEYBOARD_STATE.key_down[Key::RightArrow as usize] = false,
            75 => KEYBOARD_STATE.key_down[Key::LeftArrow as usize] = true,
            203 => KEYBOARD_STATE.key_down[Key::LeftArrow as usize] = false,
            57 => KEYBOARD_STATE.key_down[Key::Space as usize] = true,
            185 => KEYBOARD_STATE.key_down[Key::Space as usize] = false,
            _ => {}
        }
        KEYBOARD_STATE.input_version += 1;
    }
    
}

pub fn current_keyboard_state() -> KeyboardState {
    // because we do not have interrupts, we are garenteed to get a
    // fully complete set of up/down keys
    x86_64::instructions::interrupts::without_interrupts(|| {
        unsafe { KEYBOARD_STATE.clone() }
    })
}
