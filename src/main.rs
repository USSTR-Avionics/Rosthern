#![no_main]
#![no_std]

use core::arch::asm; // use arm assembly
use panic_halt as _; // panic handler

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! 
    {
    loop {}
    }

#[entry]
fn main() 
    {
    loop
        {
        unsafe
            {
            asm!("nop")
            }
        }
    }
