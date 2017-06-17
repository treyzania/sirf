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

echo -e "
	.section .data
	.globl $name
	.globl $end_lbl
	.globl $size_lbl
$name:
	.incbin \"$file\"
$end_lbl:
	.byte 0
$size_lbl:
	.long $size
" | gcc -c -x assembler -o $dest -
