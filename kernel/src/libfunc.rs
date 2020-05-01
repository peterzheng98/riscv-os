pub const UART_RHR: usize = 0;
pub const UART_THR: usize = 0;
pub const UART_IER: usize = 1;
pub const UART_FCR: usize = 2;
pub const UART_ISR: usize = 2;
pub const UART_LCR: usize = 3;
pub const UART_LSR: usize = 5;

pub struct UART{
    mem_addr: usize;
}

impl UART{
    pub fn init(&mut self){
        
    }
}