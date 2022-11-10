#![allow(dead_code)]
#![no_std]
#![feature(abi_x86_interrupt)]

pub mod kernel {
    pub mod graphics;  
    pub mod interrupts;

    pub fn init() {
        interrupts::init();
    }
 
}

pub mod game;


