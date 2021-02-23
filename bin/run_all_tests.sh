DIR=$(pwd)/bin/$1/tests

for file in "$DIR"/*
do
    "$file"
done
