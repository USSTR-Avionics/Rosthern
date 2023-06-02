static mut MESSAGES_QUEUE: [u8; 1] = [0; 1];

/// This function returns a pointer to a shared memory region which is an array of u8, with a size
/// of 10. This is the memory region that is used to pass messages between tasks. Be sure to pass
/// this pointer downstream to the C runtime, as it will be inaccessible later due to a circular dependency
pub fn get_message_array_pointer() -> *mut u8
    {
    unsafe { MESSAGES_QUEUE.as_mut_ptr() }
    }

/// Subscribe a downstream C function to this to snoop on the messages in the message queue
/// This will check if a certain flag is set and if so, it will call the C function
#[no_mangle]
pub fn snoop() 
    {
    unsafe
        {
        match MESSAGES_QUEUE[0]
            {
            0 => {
                // C func call
                },
            1 => {
                // C func call
                },
            _ => 
                {
                panic!("postman snooped on invalid message");
                }
            }
        }
    }
