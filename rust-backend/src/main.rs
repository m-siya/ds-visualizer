mod animation;
use animation::*;

buffer!();
// TODO : make this buffer importable

fn main() {
    let mut stack = create_stack!();
    (0..10).for_each(|x| stack.push(format!("item {}", x)));
    println!("{:?}", stack);

    let stack = reverse(stack);
    println!("{:?}", stack);
}

fn reverse<T>(mut stack: AnimatedStack<T>) -> AnimatedStack<T>
where
    T: std::fmt::Debug,
{
    let mut reversed = create_stack!();

    while !stack.is_empty() {
        reversed.push(stack.pop().unwrap());
    }
    reversed
}
