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

It's a library for interacting with SIRF files that are `mmap`ped into memory.
