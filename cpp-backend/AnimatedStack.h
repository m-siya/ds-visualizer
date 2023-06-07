#include <stack>
#include <iostream>
#include "utils.h"
#include "Line.h"

namespace DS_Visualizer
{
    template <class T>
    // use the standard library stack implementation and make this a wrapper around it.
    class AnimatedStack
    {
    public:
        AnimatedStack();

        void push(T elem);

        void pop();

        T top();

        bool empty();

        int size();

    private:
        std::stack<T> stack;
        Line<DS_Visualizer::Option<T>> pops;
        Line<T> pushes;
    };
}
namespace DS_Visualizer
{
    template <class T>
    AnimatedStack<T>::AnimatedStack() : stack(std::stack<T>())
    {
        pushes.setDSName("stack");
        pushes.setDsId(reinterpret_cast<uint64_t>(&stack));
        pushes.setOperation("push");
        pops.setDSName("stack");
        pops.setDsId(reinterpret_cast<uint64_t>(&stack));
        pops.setOperation("pop");
    }

    template <class T>
    void AnimatedStack<T>::push(T elem)
    {
        std::cout << "Inside AnimatedStack::push, elem = " << elem << std::endl;
        pushes.setElemType(elem);
        pushes.write();
        stack.push(elem);
    }

    template <class T>
    void AnimatedStack<T>::pop()
    {
        if (stack.empty())
        {
            auto none = Option<T>();
            pops.setElemType(none);
            pops.write();
        }
        else
        {
            T &top = stack.top();
            pops.setElemType(Option<T>(top));
            pops.write();
        }

        // the reason we don't check for empty stack here is because
        // we want to emulate the behaviour of std::stack when top() or pop() is called on an empty stack.
        stack.pop();
    }

    template <class T>
    T AnimatedStack<T>::top()
    {
        return stack.top();
    }

    template <class T>
    bool AnimatedStack<T>::empty()
    {
        return stack.empty();
    }

    template <class T>
    int AnimatedStack<T>::size()
    {
        return this->stack->size();
    }
}
