from .animation_buffer import AnimationBuffer
import pyfastrand
from collections import deque
class AnimatedStack:

    def __init__(self, buffer = None):
        self.name = "stack"
        self.id = pyfastrand.pcg32()
        self.contents = deque()
        self.buffer = AnimationBuffer() if not buffer else buffer

    def push_back(self, element):
        self.buffer.write(data_structure = self.name, operation="push", id = self.id, data = element)
        self.contents.append(element)

    def pop_back(self):
        element = self.contents[-1]
        self.buffer.write(data_structure = self.name, operation="pop", id=self.id, data = element)
        return self.contents.pop()

    def peek_back(self):
        element = self.contents[-1]
        self.buffer.write(data_structure=self.name, operation = "peek", id = self.id, data = element)
        return element

    def top(self):
        return self.peek_back()

    def len(self):
        length = len(self.contents)
        self.buffer.write(data_structure=self.name, operation="len", id = self.id, data = length)
        return length
    
    def size(self):
        return self.len()
    
    def is_empty(self):
        self.buffer.write(data_structure=self.name, operation = "is_empty", id = self.id, data = not self.contents)
        return not self.contents
    
    def clear(self):
        self.buffer.write(data_structure=self.name, operation = "clear", id = self.id, data = len(self.contents))
        self.contents.clear()


