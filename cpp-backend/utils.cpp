#include "utils.h"

namespace DS_Visualizer
{
    const char *UnwrapException::what() const throw()
    {
        return "Option is empty!";
    }
};