#![no_main]
#![no_std]

use cortex_m::peripheral::{syst::SystClkSource};
use cortex_m_rt::{entry, exception};
use core::mem::MaybeUninit;
use panic_halt as _;
use core::arch::asm; // use arm assembly



static mut TASK_QUEUE: MaybeUninit<TaskQueue> = MaybeUninit::uninit();
static mut REGISTERS: [u32; 8] = [0; 8];

/// this struct keeps track of the tasks that are running
/// tasks: array of function pointers to the tasks
/// current_task: index of the current task
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
    unimplemented!()
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

/// Call on the main() function for the C code that sits on top of the RTOS layer
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

/// Call on the engine's main() function for the C code that sits on top of the RTOS layer
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
