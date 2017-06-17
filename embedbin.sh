#!/bin/bash

file=$1
name=$2

if [ -z "$name" ]; then
	name=$(echo $file | sed 's/\..*$//')
fi

dest="$name.o"

echo -e "\
\t.section .data\n\
\t.globl $name\n\
\t.globl fin_$name\n\
$name:\n\
\t.incbin \"$file\"\n\
fin_$name:\n\
\t.byte 0" \
| gcc -c -x assembler -o $dest -
