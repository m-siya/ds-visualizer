# Compiler
CXX := g++

# Compiler flags
CXXFLAGS := 

# Source files
SRCS_CPP := main.cpp utils.cpp LogFileHandler.cpp
# SRCS_H := Line.h AnimatedStack.h

# Object files
OBJS := $(SRCS_CPP:.cpp=.o) # $(SRCS_H:.h=.o)

# Executable
EXEC := main

.PHONY: all clean

all: $(EXEC)

$(EXEC): $(OBJS)
	$(CXX) $(CXXFLAGS) $(OBJS) -o $(EXEC)

%.o: %.cpp
	$(CXX) $(CXXFLAGS) -c $< -o $@

%.o: %.h
	$(CXX) $(CXXFLAGS) -c $< -o $@

clean:
	rm -f $(OBJS) $(EXEC)
