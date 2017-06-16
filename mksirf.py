#!/usr/bin/env python

import os
import os.path as paths
import sys
import struct

path = '.'
dest = paths.basename(paths.abspath('.')) + '.sird'

if len(sys.argv) > 1:
    path = sys.argv[1]
    dest = paths.basename(paths.abspath(path)) + '.sird'

# Now we start to write the files.
with open(dest, 'wb') as output:

    # Write the magic number.
    output.write(b'SIRF')

    # Tabulate the files we have to write
    records = []
    for a, b, filenames in os.walk(path):
        for f in filenames:
            records.append(paths.normpath(paths.join(paths.relpath(a, start=path), f))) # Ugly, fix this eventually.s
    output.write(struct.pack('>I', len(records)))

    # Actually pack the files.
    for r in records:
        rpath = paths.join(path, r)
        size = paths.getsize(rpath)
        header = struct.pack('>QH', size, len(r))
        output.write(header)
        output.write(unicode(r, 'utf-8'))
        with open(rpath, 'rb') as rec:
            output.write(rec.read())
