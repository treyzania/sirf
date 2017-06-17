#!/bin/bash

file=$1
name=$2

if [ -z "$name" ]; then
	name=$(echo $file | sed 's/\..*$//')
fi

dest="$name.o"

end_lbl=$name'_end'
size_lbl=$name'_len'
size=$(ls $file | awk '{ print $5; }')

echo -e "\
\t.section .data\n\
\t.globl $name\n\
\t.globl $end_lbl\n\
\t.globl $size_lbl\n\
$name:\n\
\t.incbin \"$file\"\n\
$end_lbl:\n\
\t.byte 0\n\
$size_lbl:\n\
\t.long $size\n" \
| gcc -c -x assembler -o $dest -
