use super::animation_buffer::AnimationBuffer;
extern crate fastrand;

pub struct AnimatedStack<'a, T> {
    id: usize,
    contents: Vec<T>,
    pub buffer: &'a AnimationBuffer,
}

#[allow(dead_code)]
impl<'a, T: std::fmt::Debug + std::clone::Clone> AnimatedStack<'a, T> {
    pub fn new(buffer: &'a AnimationBuffer) -> AnimatedStack<T> {
        Self {
            id: fastrand::usize(..),
            contents: Vec::new(),
            buffer,
        }
    }
    pub fn push(&mut self, element: T) {
        self.buffer
            .write(format!("stack_{}:push({:?})", self.id, element.clone()));
        self.contents.push(element);
    }
    pub fn push_back(&mut self, element: T) {
        self.push(element);
    }
    pub fn pop(&mut self) -> Option<T> {
        let val = self.contents.pop();
        self.buffer
            .write(format!("stack_{}:pop({:?})", self.id, val));
        val
    }
    pub fn pop_back(&mut self) -> Option<T> {
        self.pop()
    }
    pub fn peek(&self) -> Option<&T> {
        let val = self.contents.last();
        self.buffer
            .write(format!("stack_{}:peek({:?})", self.id, val));
        val
    }
    pub fn peek_back(&self) -> Option<&T> {
        self.peek()
    }
    pub fn top(&self) -> Option<&T> {
        self.peek()
    }
    pub fn len(&self) -> usize {
        self.buffer
            .write(format!("stack_{}:len({})", self.id, self.contents.len()));
        self.contents.len()
    }
    pub fn size(&self) -> usize {
        self.len()
    }
    pub fn is_empty(&self) -> bool {
        self.buffer.write(format!(
            "stack_{}:is_empty({})",
            self.id,
            self.contents.is_empty()
        ));
        self.contents.is_empty()
    }
    pub fn clear(&mut self) {
        self.buffer
            .write(format!("stack_{}:clear({})", self.id, self.contents.len()));
        self.contents.clear();
    }
}
