#![no_main]
#![no_std]

use cortex_m::peripheral::{syst::SystClkSource};
use cortex_m_rt::{entry, exception};
use core::mem::MaybeUninit;
use panic_halt as _;
use core::arch::asm; // use arm assembly



static mut TASK_QUEUE: MaybeUninit<TaskQueue> = MaybeUninit::uninit();
static mut REGISTERS: [u32; 8] = [0; 8];

struct TaskQueue
    {
    tasks: [fn() -> !; 2],
    current_task: usize,
    }

impl TaskQueue
    {
    fn new() -> Self
        {
        Self
            {
            tasks: [main_task, engine_task],
            current_task: 0,
            }
        }
    }

fn switch_rtos_context()
    {
    let tq = unsafe { &mut TASK_QUEUE.assume_init_mut() };
    let current_task = tq.current_task;
    let next_task = if current_task == 0 { 1 } else { 0 };

    // Save the current task's context
    unsafe 
        {
        asm!(
            "mrs r0, psp",
            "stmia r0!, {{r4-r11}}", // save r4-r11 to stack
            "mov r1, $0" : : "r" (&mut REGISTERS as *mut [u32; 8]) : "r1" : "volatile", "intel",
            );
        REGISTERS.copy_from_slice(&[current_task as u32]);
        }

    // Switch to the next task
    tq.current_task = next_task;

    // Restore the next task's context
    let next_task_registers = unsafe { &REGISTERS[(next_task * 8)..(next_task * 8 + 8)] };

    unsafe 
        {
        asm!(
            "mov r1, $0" :: "r" (next_task_registers as *const [u32; 8]) : "r1" : "volatile", "intel",
            "ldmia r0!, {{r4-r11}}", // restore r4-r11 from stack
            "msr psp, r0",
            "bx lr",
            options(noreturn)
            );
        }

    }

#[exception]
fn SysTick()
    {
    switch_rtos_context();
    }

fn setup_systick(syst: &mut cortex_m::peripheral::SYST, clock_cycles: u32)
    {
    // Set reload value to trigger every clock_cycles(var) ticks
    syst.set_reload(clock_cycles);

    // Clear the current value
    syst.clear_current();

    // Enable SysTick timer and its interrupt
    syst.enable_counter();
    syst.enable_interrupt();
    }

#[entry]
fn setup() -> !
    {
    let interrupt_clock_cycles = 10_000; // 10 KHz
                                         //
    let p = cortex_m::Peripherals::take().unwrap();
    let mut syst = p.SYST;
                                         //
    syst.set_clock_source(SystClkSource::Core);
    setup_systick(&mut syst, interrupt_clock_cycles);

    unsafe
        {
        let tq = MaybeUninit::new(TaskQueue::new());
        tq.assume_init().tasks[0]();
        };
    }

#[no_mangle]
fn main_task() -> !
    {
    loop
        {
        // turn on LED
        unsafe
            {
            asm!("nop")
            }
        }
    }

#[no_mangle]
fn engine_task() -> !
    {
    loop
        {
        // turn off LED
        unsafe
            {
            asm!("nop")
            }
        }
    }
