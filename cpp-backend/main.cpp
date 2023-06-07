#include "AnimatedStack.h"

int main()
{
    DS_Visualizer::AnimatedStack<int> stack;
    stack.push(1);
    stack.push(40);
    stack.push(50);
    stack.pop();
    stack.pop();
    stack.pop();
    stack.pop();
    return 0;
}