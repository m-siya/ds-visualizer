use std::fmt::Debug;

use super::{AnimationBuffer, AnimationBufferWrite, DataStructure, NewInstance, AnimatedFromIterator};

pub struct AnimatedStack<T: std::fmt::Debug> {
    contents: Vec<T>,
    pub buffer: AnimationBuffer,
}

impl<T: Debug> Debug for AnimatedStack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.contents.fmt(f)
    }
}

impl<T: Debug> NewInstance<T> for AnimatedStack<T> {
    fn new(buffer: &AnimationBuffer) -> Self {
        Self {
            contents: Vec::new(),
            buffer: buffer.with_ds(DataStructure::Stack),
        }
    }
}

impl<T: Debug> AnimatedFromIterator<T> for AnimatedStack<T> {
    fn from_iter<I: IntoIterator<Item = T>>(buffer: &AnimationBuffer, iter: I) -> Self {
      Self{
          contents: Vec::from_iter(iter),
          buffer: buffer.with_ds(DataStructure::Stack),
      } 
    }
} 
#[allow(dead_code)]
impl<T: Debug> AnimatedStack<T> {
    /* pub fn new(buffer: &AnimationBuffer) -> AnimatedStack<T> {
        Self {
            contents: Vec::new(),
            buffer: buffer.with_ds(DataStructure::Stack),
        }
    } */
    pub fn push(&mut self, element: T) {
        self.buffer.write("push", &element);
        self.contents.push(element);
    }
    pub fn push_back(&mut self, element: T) {
        self.push(element);
    }
    pub fn pop(&mut self) -> Option<T> {
        let val = self.contents.pop();
        self.buffer.write("pop", &val);
        val
    }
    pub fn pop_back(&mut self) -> Option<T> {
        self.pop()
    }
    pub fn peek(&self) -> Option<&T> {
        let val = self.contents.last();
        self.buffer.write("peek", &val);
        val
    }
    pub fn peek_back(&self) -> Option<&T> {
        self.peek()
    }
    pub fn top(&self) -> Option<&T> {
        self.peek()
    }
    pub fn len(&self) -> usize {
        self.buffer.write("len", &self.contents.len());
        self.contents.len()
    }
    pub fn size(&self) -> usize {
        self.len()
    }
    pub fn is_empty(&self) -> bool {
        self.buffer.write("is_empty", &self.contents.is_empty());
        self.contents.is_empty()
    }
    pub fn clear(&mut self) {
        self.buffer.write("clear", &self.contents.len());
        self.contents.clear();
    }
}
