from .animation_buffer import AnimationBuffer
import pyfastrand
from typing import Any
from collections import deque

class AnimatedStack:

    def __init__(self, buffer: AnimationBuffer = None) -> None:
        self.name: str = "stack"
        self.id: int = pyfastrand.pcg32()
        self.contents: deque = deque()
        self.buffer: AnimationBuffer = AnimationBuffer() if not buffer else buffer

    def push_back(self, element: Any) -> None:
        self.buffer.write(data_structure = self.name, operation="push", id = self.id, data = element)
        self.contents.append(element)

    def pop_back(self) -> Any:
        element = self.contents[-1]
        self.buffer.write(data_structure = self.name, operation="pop", id=self.id, data = element)
        return self.contents.pop()

    def peek_back(self) -> Any:
        element = self.contents[-1]
        self.buffer.write(data_structure=self.name, operation = "peek", id = self.id, data = element)
        return element

    def top(self) -> Any:
        return self.peek_back()

    def len(self) -> int:
        length = len(self.contents)
        self.buffer.write(data_structure=self.name, operation="len", id = self.id, data = length)
        return length
    
    def size(self) -> int:
        return self.len()
    
    def is_empty(self) -> bool:
        self.buffer.write(data_structure=self.name, operation = "is_empty", id = self.id, data = not self.contents)
        return not self.contents
    
    def clear(self) -> None:
        self.buffer.write(data_structure=self.name, operation = "clear", id = self.id, data = len(self.contents))
        self.contents.clear()


