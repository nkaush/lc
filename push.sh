#!/usr/bin/env bash

if [ $# -ne 1 ]
then
    echo "Usage:   ./push.sh XXXX"
    echo "Example: ./push.sh 0001"
    exit 1
fi

git add $1
git commit -m "$1"
git push