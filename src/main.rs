#![no_main]
#![no_std]

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use core::mem::MaybeUninit;
use panic_halt as _;
use core::arch::asm;



static mut TASK_QUEUE: MaybeUninit<TaskQueue> = MaybeUninit::uninit();
static mut REGISTERS_NEXT: [u32; 15] = [0; 15];
static mut REGISTERS_PREV: [u32; 15] = [0; 15];
static mut MESSAGES_QUEUE: [u8; 10] = [0; 10];


/// This function returns a pointer to a shared memory region which is an array of u8, with a size
/// of 10. This is the memory region that is used to pass messages between tasks. Be sure to pass
/// this pointer downstream to the C runtime, as it will be inaccessible later due to a circular dependency
fn get_common_memory_pointer() -> *mut u8
    {
    unsafe { MESSAGES_QUEUE.as_mut_ptr() }
    }


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
    // TODO: 
    // See what all registers need to be saved
    // See what all registers need to be restored

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

    // load next_task array into prev_task array
    unsafe
        {
        REGISTERS_PREV.copy_from_slice(&REGISTERS_NEXT);
        // for i in 0..REGISTERS_PREV.len()
        //     {
        //     REGISTERS_PREV[i] = REGISTERS_NEXT[i];
        //     }
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

/// This is the entry point for the RTOS
/// It sets up the system timer and then calls the first task
/// This function should never returns
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
        TASK_QUEUE.write(TaskQueue::new());
        TASK_QUEUE.assume_init_mut().tasks[0]();
        };
    }

/// Call on the main() function for the C code that sits on top of the RTOS layer
/// Be sure to pass on the common pointer downstream to the C runtime
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
/// Be sure to pass on the common pointer downstream to the C runtime
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
