#!/bin/bash

if [[ -z $1 ]]; then
    echo "Usage: ./fuzz.sh N"
    exit 1
fi
N=$1

echo "Fuzzing with ${N}K inputs. (Press Enter to start)"
read

while true; do
    dd if=/dev/urandom of=./testdata/input.txt bs=1024 count=$N 2> /dev/null
    # echo $(md5sum ./testdata/input.txt) $(stat -c %s ./testdata/input.txt)
    ./target/release/divsufsort || break
done
