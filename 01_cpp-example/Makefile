######################################################################
# @author      : Enno Boland (mail@eboland.de)
# @file        : Makefile
# @created     : Monday Jun 19, 2023 17:23:35 CEST
######################################################################

CXX=g++
CFLAGS=-Wall -Wextra -Werror -pedantic -std=c++11 -fsanitize=address

OBJ = main.o 

all: run

%.o: %.c
	$(CXX) -c -o $@ $< $(CFLAGS)

cpp-example: $(OBJ)
	$(CXX) -o $@ $^ $(CFLAGS)

.PHONY: clean

clean:
	rm -f *.o ./cpp-example

run: cpp-example
	./cpp-example

valgrind: cpp-example
	valgrind ./cpp-example
