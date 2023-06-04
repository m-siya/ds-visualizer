/* use std::collections::VecDeque;

use super::animation_buffer::AnimationBuffer;
extern crate fastrand;

pub struct AnimatedQueue<'a, T> {
    id: usize,
    contents: VecDeque<T>,
    pub buffer: &'a AnimationBuffer,
}

#[allow(dead_code)]
impl<'a, T: std::fmt::Debug + std::clone::Clone> AnimatedQueue<'a, T> {
    pub fn new(buffer: &'a AnimationBuffer) -> AnimatedQueue<T> {
        Self {
            id: fastrand::usize(..),
            contents: VecDeque::new(),
            buffer,
        }
    }

    pub fn push_back(&mut self, element: T) {
        self.buffer
            .write(format!("queue_{}:pushback({:?})", self.id, element.clone()));
        self.contents.push_back(element);
    }

    pub fn push_front(&mut self, element: T) {
        self.buffer.write(format!(
            "queue_{}:pushfront({:?})",
            self.id,
            element.clone()
        ));
        self.contents.push_front(element);
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let val = self.contents.pop_back();
        self.buffer
            .write(format!("queue_{}:popback({:?})", self.id, val));
        val
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let val = self.contents.pop_front();
        self.buffer
            .write(format!("queue_{}:popfront({:?})", self.id, val));
        val
    }
    pub fn peek_back(&self) -> Option<&T> {
        let val = self.contents.back();
        self.buffer
            .write(format!("queue_{}:peekback({:?})", self.id, val));
        val
    }
    pub fn peek_front(&self) -> Option<&T> {
        let val = self.contents.front();
        self.buffer
            .write(format!("queue_{}:peekfront({:?})", self.id, val));
        val
    }
    pub fn len(&self) -> usize {
        self.buffer.write(format!(
            "queue_{}:peekfront({:?})",
            self.id,
            self.contents.len()
        ));
        self.contents.len()
    }
    pub fn size(&self) -> usize {
        self.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.write(format!(
            "queue_{}:is_empty({:?})",
            self.id,
            self.contents.is_empty()
        ));
        self.contents.is_empty()
    }

    pub fn clear(&mut self) {
        self.buffer
            .write(format!("queue_{}:clear({})", self.id, self.contents.len()));
        self.contents.clear();
    }
} */
