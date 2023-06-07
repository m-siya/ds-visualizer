#include <cstdint>
#include <iostream>
#include "LogFileHandler.h"

namespace DS_Visualizer
{
    // really simple class, okay for it to be defined in the header file.
    template <class T>
    class Line
    {
    public:
        void setDSName(const char *ds_name);
        void setOperation(const char *operation);
        void setDsId(uint64_t ds_id);
        void setElemType(const T &elem_type);

        void write();

    private:
        const char *ds_name;
        const char *operation;
        uint64_t ds_id;
        const T *elem_type;
    };
}
namespace DS_Visualizer
{
    template <class T>
    void Line<T>::setDSName(const char *ds_name)
    {
        this->ds_name = ds_name;
    }
    template <class T>
    void Line<T>::setOperation(const char *operation)
    {
        this->operation = operation;
    }

    template <class T>
    void Line<T>::setDsId(uint64_t ds_id)
    {
        this->ds_id = ds_id;
    }
    template <class T>
    void Line<T>::setElemType(const T &elem)
    {
        this->elem_type = &elem;
    }

    template <class T>
    void Line<T>::write()
    {
        LogFileHandler &logFile = LogFileHandler::getLogFile();
        std::cout << "The value that I received: " << *elem_type << std::endl;
        logFile << ds_name << '_' << ds_id << ':' << operation << '(' << *elem_type << ')' << '\n';
        }
}