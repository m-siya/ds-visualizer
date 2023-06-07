#pragma once
#include <exception>
#include <iostream>

namespace DS_Visualizer
{
    class UnwrapException : public std::exception
    {
    public:
        const char *what() const throw();
    };

    template <class T>
    class Option
    {
    private:
        T value;
        bool isPresent;

    public:
        Option();
        Option(T value);
        bool isEmpty();

        T unwrap();

        template <class U>
        friend std::ostream &operator<<(std::ostream &os, const Option<U> &option);
    };
}

namespace DS_Visualizer
{
    template <class T>
    Option<T>::Option() : isPresent(false)
    {
        // SAFETY: It's fine to leave value uninitialized here because it is guarded by the isPresent boolean.
        // so the unitiiialized value will never be accessed.
    }
    template <class T>
    Option<T>::Option(T value) : value(value), isPresent(true) {}

    template <class T>
    bool Option<T>::isEmpty()
    {
        return !isPresent;
    }

    template <class T>
    T Option<T>::unwrap()
    {
        if (isPresent)
        {
            return value;
        }
        else
        {
            throw UnwrapException();
        }
    }

    template <class U>
    std::ostream &operator<<(std::ostream &os, const Option<U> &option)
    {
        if (option.isPresent)
        {
            os << "Some(" << option.value << ")";
        }
        else
        {
            os << "None";
        }
        return os;
    }
}