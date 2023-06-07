#include "LogFileHandler.h"

// This FileHandler as of now IS NOT THREAD SAFE. Its beneficiaries use as-is without locks and as a singleton.
// It must not be run on multiple threads.
namespace DS_Visualizer
{
    LogFileHandler::LogFileHandler(std::string filename = DEFAULT_FILENAME)
        : filename(filename)
    {
        this->file.open(filename, std::ios::out | std::ios::app);
        file << FILE_PREFIX << "\n";
    }

    void LogFileHandler::writeLine(std::string line)
    {
        this->file << line << std::endl;
    }

    LogFileHandler::~LogFileHandler()
    {
        this->file.close();
    }

    LogFileHandler &LogFileHandler::getLogFile()
    {
        static LogFileHandler logFile;
        return logFile;
    }
};
