# SIRF : The Simple Immutable Record Format

SIRF is a format I created so that I could easily package multiple files
together in a way that would make it easy to wrap them in some metadata and
link the raw data into a kernel boot image.  It's similar to a key-value store
in that "records" have a name and contents.  There is no file hierarchy and no
file permissions, but it serves a similar purpose to `initrd` and the records
are supposed to be named with POSIX paths.

## mksirf.py

`mksirf.py` is a small script that converts a file directory into a SIRF
archive.  It is named the same as the directory name, but with the `.sird`
extension atteched onto the end.

## libsirf.c

It's a library for interacting with SIRF files that are already loaded into
memory.  It doesn't matter if they're `mmap`ed into memory or if they are
already loaded as part of the binary.

This goes very well with `embedbin.sh`, as you should see.

## embedbin.sh

A shell script to turn arbitrary files (in this case, sirf data) into object
files that can be referenced by C code.  It's a little bit strange, but it's
actually rather simple.

Let's say you have a file called "foo.sird".  This generates an ELF object that
exports the following symbols:

* `foo` - Beginning of foo data.
* `foo_end` - 0 byte appended to foo data.
* `foo_size` - Length of foo data in an unsigned 32-bit integer.

You can reference these symbols from C code by putting these statements
somewhere in your C code, probably in a header somewhere.

```c
extern uint8_t foo; // char or unsigned char also work, depending on your data
extern uint8_t foo_end; // Should always be 0, but here for reasons.
extern uint32_t foo_size;
```

Since these are not pointers, you have to take "address of" for some things.

```c
#include <stdio.h>

// plaintext.o
extern char plaintext;
extern uint32_t plaintext_size;

int main(void) {
	printf("%i bytes: %s", plaintext_size, &plaintext); // Note the &.
	return 0;
}
```

This couples well with `libsirf` as you can just say `sirf_record(&foo, "bar")`
to get the record "bar" from whatever sird data is loaded at `foo`.
