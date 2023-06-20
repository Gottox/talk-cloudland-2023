######################################################################
# @author      : Enno Boland (mail@eboland.de)
# @file        : Makefile
# @created     : Monday Jun 19, 2023 17:23:35 CEST
######################################################################

IDIR =./include
CXX=g++
#CFLAGS=-Wall -Wextra -Werror -pedantic -std=c++11 -fsanitize=address
CFLAGS=-Wall -Wextra -Werror -pedantic -std=c++11

OBJ = main.o 

all: run

%.o: %.c
	$(CXX) -c -o $@ $< $(CFLAGS)

cpp-crash: $(OBJ)
	$(CXX) -o $@ $^ $(CFLAGS)

.PHONY: clean

clean:
	rm -f *.o ./cpp-crash

run: cpp-crash
	./cpp-crash

valgrind: cpp-crash
	valgrind ./cpp-crash
