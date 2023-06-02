#![no_std]
#![no_main]



use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_semihosting::{debug, hprintln};
use cortex_m_rt::{entry, exception};
use core::mem::MaybeUninit;
use rosthern::postman;
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
use core::arch::asm;
use cortex_m::asm;



static mut TASK_QUEUE: MaybeUninit<TaskQueue> = MaybeUninit::uninit();
/// Registers that need to be saved and restored
/// r15 | pc | Program Counter
/// r14 | lr | Link Register
/// r13 | sp | Stack Pointer
/// r12 | ip | Intra-Procedure-call scratch register
/// r11 | v8 | ARM-state variale register 8
/// r10 | v7/sl | ARM-state variale register 7, stack limit pointer in stack-checked varients
/// r9  | v6/sb | ARM-state variale register 6, static base in RWPI varients
/// r8  | v5 | ARM-state variale register 5
/// r7  | v4 | ARM-state variale register 4
/// r6  | v3 | ARM-state variale register 3
/// r5  | v2 | ARM-state variale register 2
/// r4  | v1 | ARM-state variale register 1
/// r3  | a4 | ARM-state argument register 4
/// r2  | a3 | ARM-state argument register 3
/// r1  | a2 | ARM-state argument register 2
/// r0  | a1 | ARM-state argument register 1
static mut REGISTERS_PREV: [u32; 15] = [0; 15];
static mut REGISTERS_CURR: [u32; 15] = [0; 15];
static mut REGISTERS_NEXT: [u32; 15] = [0; 15];

/// this struct holds the tasks that need to be run
/// tasks: array of function pointers to the tasks
/// current_task: index of the current task
#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct TaskQueue
    {
    tasks: [fn() -> !; 2],
    curr: usize,
    run_second: bool,
    }

impl TaskQueue
    {
    fn new() -> Self
        {
        Self
            {
            tasks: [main_task, engine_task],
            curr: 0,
            run_second: false
            }
        }
    fn get_tasks(&self) -> &[fn() -> !]
        {
        &self.tasks
        }
    }

/// The processor pushes 8 registers PSR, PC, LR, R12, R3, R2 R1, and R0 onto the stack on an exception. 
/// Then the exception routine is executed. On exit from the exception, 
/// the processor pops 8 words(the same ones those where pushed) from the stack and loads them onto 
/// the respective CPU registers(in the same order as they where saved)
/// We have to only save and restore the rest of the registers(R4, R5, R6, R7, R8, R9, R10 & R11) 
/// within the interrupt. Since SP is directly stored in the TCB, we don’t have to push it to the stack.
fn switch_rtos_context()
    {
    postman::snoop();

    let p = cortex_m::Peripherals::take().unwrap();
    let mut syst = p.SYST;
    syst.set_clock_source(SystClkSource::Core);

    main_task();


//    unsafe {
//        if TASK_QUEUE.assume_init_mut().curr == 0
//            {
//            TASK_QUEUE.assume_init_mut().curr = 1;
//            engine_task();
//            }
//        else
//            {
//            TASK_QUEUE.assume_init_mut().curr = 0;
//            main_task();
//            }
//    }

    /*
    hprintln!("switching context").unwrap();

    // save all the registers of the current task into an array
    unsafe
        {
        asm!("mov {0}, r0 ", out(reg) REGISTERS_CURR[0]);
        asm!("mov {0}, r1 ", out(reg) REGISTERS_CURR[1]);
        asm!("mov {0}, r2 ", out(reg) REGISTERS_CURR[2]);
        asm!("mov {0}, r3 ", out(reg) REGISTERS_CURR[3]);
        asm!("mov {0}, r4 ", out(reg) REGISTERS_CURR[4]);
        asm!("mov {0}, r5 ", out(reg) REGISTERS_CURR[5]);
        asm!("mov {0}, r6 ", out(reg) REGISTERS_CURR[6]);
        asm!("mov {0}, r7 ", out(reg) REGISTERS_CURR[7]);
        asm!("mov {0}, r8 ", out(reg) REGISTERS_CURR[8]);
        asm!("mov {0}, r9 ", out(reg) REGISTERS_CURR[9]);
        asm!("mov {0}, r10", out(reg) REGISTERS_CURR[10]);
        asm!("mov {0}, r11", out(reg) REGISTERS_CURR[11]);
        asm!("mov {0}, r12", out(reg) REGISTERS_CURR[12]);
        }

    unsafe
        {
        hprintln!("on switch {:?}", REGISTERS_PREV).unwrap();
        hprintln!("on switch {:?}", REGISTERS_CURR).unwrap();
        }

    // load all the registers of the next task from a array
    unsafe
        {
        asm!("mov r0,  {0}", in(reg) REGISTERS_PREV[0]);
        asm!("mov r1,  {0}", in(reg) REGISTERS_PREV[1]);
        asm!("mov r2,  {0}", in(reg) REGISTERS_PREV[2]);
        asm!("mov r3,  {0}", in(reg) REGISTERS_PREV[3]);
        asm!("mov r4,  {0}", in(reg) REGISTERS_PREV[4]);
        asm!("mov r5,  {0}", in(reg) REGISTERS_PREV[5]);
        asm!("mov r6,  {0}", in(reg) REGISTERS_PREV[6]);
        asm!("mov r7,  {0}", in(reg) REGISTERS_PREV[7]);
        asm!("mov r8,  {0}", in(reg) REGISTERS_PREV[8]);
        asm!("mov r9,  {0}", in(reg) REGISTERS_PREV[9]);
        asm!("mov r10, {0}", in(reg) REGISTERS_PREV[10]);
        asm!("mov r11, {0}", in(reg) REGISTERS_PREV[11]);
        asm!("mov r12, {0}", in(reg) REGISTERS_PREV[12]);
        }

    unsafe 
        {
        REGISTERS_PREV.copy_from_slice(&mut REGISTERS_CURR);
        }

    unsafe
        {
        hprintln!("on leaving switch {:?}", REGISTERS_CURR).unwrap();
        hprintln!("on leaving switch {:?}", REGISTERS_PREV).unwrap();
        }
    
    unsafe
        {
        if !TASK_QUEUE.assume_init_mut().run_second
            {
            TASK_QUEUE.assume_init_mut().run_second = true;
            TASK_QUEUE.assume_init_mut().get_tasks()[1]();
            }
        }
    */
    }

#[exception]
fn SysTick()
    {
    hprintln!("SysTick------------------------------").unwrap();
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
    hprintln!("Hello, world!").unwrap();

    let interrupt_clock_cycles = 10_000; // 10 KHz

    let p = cortex_m::Peripherals::take().unwrap();
    let mut syst = p.SYST;

    syst.set_clock_source(SystClkSource::Core);
    setup_systick(&mut syst, interrupt_clock_cycles);

    unsafe
        {
        TASK_QUEUE.write(TaskQueue::new());
        TASK_QUEUE.assume_init_mut().get_tasks()[0]();
        };
    }

/// Call on the main() function for the C code that sits on top of the RTOS layer
/// Be sure to pass on the common pointer downstream to the C runtime
#[no_mangle]
fn main_task() -> !
    {
    let ptr = postman::get_message_array_pointer();
    // c_func(ptr)
    loop
        {
        asm::nop();
        // turn on LED
        hprintln!("main_task").unwrap()
        }
    }

/// Call on the engine's main() function for the C code that sits on top of the RTOS layer
/// Be sure to pass on the common pointer downstream to the C runtime
#[no_mangle]
fn engine_task() -> !
    {
    let ptr = postman::get_message_array_pointer();
    // c_func(ptr)
    loop
        {
        asm::nop();
        // turn off LED
        hprintln!("engine_task").unwrap();
        }
    }

