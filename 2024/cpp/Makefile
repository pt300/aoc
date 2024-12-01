CPPFLAGS = -O2 -std=c++17
CPP = g++

day1: day1.o
	$(CPP) $(CPPFLAGS) -o build/day1 day1.o

day1.o: day1/*
	$(CPP) $(CPPFLAGS) -c day1/main.cpp -o day1.o
