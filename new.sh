#!/usr/bin/env bash

NEW_DIR=$1

if [ $# -ne 2 ]
then
    echo "Usage:   ./new.sh XXXX \"Problem Name\""
    echo "Example: ./new.sh 0001 \"Two Sum\""
    exit 1
fi

if [ -d $NEW_DIR ]
then
    echo "$NEW_DIR already exists!"
    exit 1
fi

mkdir $NEW_DIR
cp Makefile $NEW_DIR/Makefile
echo "# $2" > $NEW_DIR/README.md