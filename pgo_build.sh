#!/usr/bin/env bash

rm -rf /tmp/pgo-data

mkdir -p /tmp/demos

function download_demo {
    if [ ! -f "/tmp/demos/$1.dem" ]; then
        URL=$(curl -s https://api.demos.tf/demos/$1 | grep -oP "\"(https:.*?\.dem)\"")
        URL="${URL//\\/}"
        URL="${URL//\"/}"
        echo "Downloading demo $1 from $URL"
        wget -q $URL -O /tmp/demos/$1.dem
    fi
}

function profile_demo {
    echo "/tmp/demos/$1.dem"
    ./target/release/parse_demo "/tmp/demos/$1.dem" > /dev/null
}

for i in $(seq 283600 283700); do download_demo $i; done

RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data" \
    cargo +nightly build --release --target=x86_64-unknown-linux-gnu

echo "Profiling demos"

for i in $(seq 283600 283700); do profile_demo $i; done

#llvm-profdata merge -o /tmp/pgo-data/merged.profdata /tmp/pgo-data

#RUSTFLAGS="-Cprofile-use=/tmp/pgo-data/merged.profdata" \
#    cargo +nightly build --release