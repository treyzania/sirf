.PHONY: clean

all: libsirf.o

libsirf.o:
	gcc-6 -c libsirf.c

clean:
	rm -rf *.o
	rm -rf *.sird
