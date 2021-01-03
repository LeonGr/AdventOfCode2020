echo "Running all days"

for i in {1..25}; do
    if [ $i -gt 9 ]; then
        NAME="day$i"
    else
        NAME="day0$i"
    fi

    echo "Running day $i"

    cd $NAME

    time ./target/release/day$i > /dev/null

    cd ..
done
