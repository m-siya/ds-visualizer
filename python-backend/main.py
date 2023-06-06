from animation.animation_buffer import AnimationBuffer
from animation.animated_stack import AnimatedStack

def reverse_stack(stack):
    another_stack = AnimatedStack(buffer)

    while (not stack.is_empty()):
        another_stack.push_back(stack.pop_back())
    
    return another_stack


buffer = AnimationBuffer()

if __name__ == '__main__':
    stack = AnimatedStack(buffer)
    
    for num in range(10):
        stack.push_back(num)
    
    print(stack.contents)

    stack = reverse_stack(stack)
    print(stack.contents)


