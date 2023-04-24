mod animation;
use crate::animation::animated_stack::AnimatedStack;
use crate::animation::animation_buffer::AnimationBuffer;

fn main() {
    let buffer = AnimationBuffer::new();
    let mut stack = AnimatedStack::new(&buffer);

    (0..10).for_each(|x| stack.push(x));

    reverse(stack);
}

fn reverse(mut arr: AnimatedStack<i32>) -> AnimatedStack<i32> {
    let mut reversed = AnimatedStack::new(arr.buffer);
    while !arr.is_empty() {
        reversed.push(arr.pop().unwrap());
    }
    reversed
}
