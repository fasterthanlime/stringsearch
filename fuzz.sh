#!/bin/bash -e

if [[ -z $1 ]]; then
    echo "Usage: ./fuzz.sh N"
    exit 1
fi
N=$1

cargo build --release 
echo "Using inputs of size ${N}K. (Ctrl-C to cancel)"
echo "2..."
sleep 1
echo "1..."
sleep 1
echo "Let's go!"

while true; do
    dd if=/dev/urandom of=./testdata/input.txt bs=1024 count=$N 2> /dev/null
    # echo $(md5sum ./testdata/input.txt) $(stat -c %s ./testdata/input.txt)
    ./target/release/divsufsort || break
done
