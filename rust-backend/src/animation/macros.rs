#[macro_export]
macro_rules! buffer {
    () => {
        static mut BUFFER: Option<AnimationBuffer> = None;
    };
}
#[macro_export]
macro_rules! init_buffer {
    () => {
        unsafe { BUFFER.get_or_insert_with(|| AnimationBuffer::new()) }
    };
    ($file:expr) => {
        unsafe { BUFFER.get_or_insert_with(|| AnimationBuffer::with_file($file)) }
    };
}
#[macro_export]
macro_rules! create_stack {
    () => {
        unsafe {
            let buff = BUFFER.get_or_insert_with(|| AnimationBuffer::new());
            AnimatedStack::new(buff)
        }
    };
}
