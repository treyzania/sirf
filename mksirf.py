#!/usr/bin/env python

import os
import os.path as paths
import sys
import struct

path = '.'
dest = paths.basename(paths.abspath('.')) + '.sird'

verbose = True

if len(sys.argv) > 1:
    path = sys.argv[1]
    dest = paths.expanduser(paths.basename(paths.abspath(path)))  + '.sird'

# Now we start to write the files.
with open(dest, 'wb') as output:

    # Write the magic number.
    output.write(b'SIRF')

    # Tabulate the files we have to write
    records = []
    for a, b, filenames in os.walk(path):
        for f in filenames:
            records.append(paths.normpath(paths.join(paths.relpath(a, start=path), f))) # Ugly, fix this eventually.
    output.write(struct.pack('>I', len(records)))

    if verbose:
        print 'wrote headers'

    # Actually pack the files.
    written = 0
    try:
        for r in records:
            rpath = paths.join(path, r)
            size = paths.getsize(rpath)
            header = struct.pack('>QH', size, len(r))
            output.write(header)
            output.write(unicode(r, 'utf-8') + b'\x00')
            with open(rpath, 'rb') as rec:
                if verbose:
                    print r
                output.write(rec.read())
                written += 1
    except Exception:
        print 'error:', sys.exc_info()[0]

    if verbose:
        print 'wrote', str(written), 'records'
