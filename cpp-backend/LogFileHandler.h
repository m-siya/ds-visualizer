#pragma once
#include <iostream>
#include <fstream>

namespace DS_Visualizer
{
    constexpr const char *FILE_PREFIX = "cpp_000";
    constexpr const char *DEFAULT_FILENAME = "animation.log";

    // this is to put this class inside a private context within this namespace, we only grant access to it
    // via the singleton.
    // (think about it, if the namespace has no name, how will you refer to stuff inside it 🤯)
    class LogFileHandler
    {
    public:
        LogFileHandler(std::string filename);

        void writeLine(std::string line);

        template <class T>
        LogFileHandler &operator<<(const T &t)
        {
            file << t;
            return *this;
        }

        ~LogFileHandler();

        static LogFileHandler &getLogFile();

    private:
        // C++ file handler types are already buffered so we don't need another buffer on top.
        std::string filename;
        std::ofstream file;
    };
}