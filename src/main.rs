#![no_main]
#![no_std]

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use core::mem::MaybeUninit;
use panic_halt as _;
use core::arch::asm; // use arm assembly



static mut TASK_QUEUE: MaybeUninit<TaskQueue> = MaybeUninit::uninit();
static mut REGISTERS_NEXT: [u32; 15] = [0; 15];
static mut REGISTERS_PREV: [u32; 15] = [0; 15];



/// this struct keeps track of the tasks that are running
/// tasks: array of function pointers to the tasks
/// current_task: index of the current task
#[repr(C)]
#[derive(Debug, Copy, Clone)]
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
    // save all the registers of the current task into an array
    unsafe
        {
        asm!("mov {0}, r0", out(reg) REGISTERS_NEXT[0]);
        asm!("mov {0}, r1", out(reg) REGISTERS_NEXT[1]);
        asm!("mov {0}, r2", out(reg) REGISTERS_NEXT[2]);
        asm!("mov {0}, r3", out(reg) REGISTERS_NEXT[3]);
        asm!("mov {0}, r4", out(reg) REGISTERS_NEXT[4]);
        asm!("mov {0}, r5", out(reg) REGISTERS_NEXT[5]);
        asm!("mov {0}, r6", out(reg) REGISTERS_NEXT[6]);
        asm!("mov {0}, r7", out(reg) REGISTERS_NEXT[7]);
        asm!("mov {0}, r8", out(reg) REGISTERS_NEXT[8]);
        asm!("mov {0}, r9", out(reg) REGISTERS_NEXT[9]);
        asm!("mov {0}, r10", out(reg) REGISTERS_NEXT[10]);
        asm!("mov {0}, r11", out(reg) REGISTERS_NEXT[11]);
        asm!("mov {0}, r12", out(reg) REGISTERS_NEXT[12]);
        }

    // load all the registers of the next task from a array
    unsafe
        {
        asm!("mov r0, {0}", in(reg) REGISTERS_PREV[0]);
        asm!("mov r1, {0}", in(reg) REGISTERS_PREV[1]);
        asm!("mov r2, {0}", in(reg) REGISTERS_PREV[2]);
        asm!("mov r3, {0}", in(reg) REGISTERS_PREV[3]);
        asm!("mov r4, {0}", in(reg) REGISTERS_PREV[4]);
        asm!("mov r5, {0}", in(reg) REGISTERS_PREV[5]);
        asm!("mov r6, {0}", in(reg) REGISTERS_PREV[6]);
        asm!("mov r7, {0}", in(reg) REGISTERS_PREV[7]);
        asm!("mov r8, {0}", in(reg) REGISTERS_PREV[8]);
        asm!("mov r9, {0}", in(reg) REGISTERS_PREV[9]);
        asm!("mov r10, {0}", in(reg) REGISTERS_PREV[10]);
        asm!("mov r11, {0}", in(reg) REGISTERS_PREV[11]);
        asm!("mov r12, {0}", in(reg) REGISTERS_PREV[12]);
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

    let p = cortex_m::Peripherals::take().unwrap();
    let mut syst = p.SYST;
                                         //
    syst.set_clock_source(SystClkSource::Core);
    setup_systick(&mut syst, interrupt_clock_cycles);

    unsafe
        {
        TASK_QUEUE = MaybeUninit::new(TaskQueue::new());
        TASK_QUEUE.assume_init().tasks[0]();
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
