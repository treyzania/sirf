.PHONY: clean

all: libsirf.o

libsirf.o:
	gcc -c libsirf.c

clean:
	rm -rf *.o
	rm -rf *.sird
