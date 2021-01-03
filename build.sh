echo "Calling 'cargo build --release' for all days"

for i in {1..25}; do
    if [ $i -gt 9 ]; then
        cd "day$i"
    else
        cd "day0$i"
    fi
    echo "Compiling day $i"
    cargo build --release
    cd ..
done

