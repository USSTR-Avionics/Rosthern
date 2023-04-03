use cortex_m_semihosting::hio;
use core::fmt::Write;


pub fn println(output_str: &str) -> Result<(), core::fmt::Error> 
    {
    let mut stdout = hio::hstdout().map(|_| core::fmt::Error);

    write!(output_str)?;

    Ok(())
    }
