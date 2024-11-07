#!/bin/sh

REF_OUT="trash/.original_fun.out"
TEST_OUT="trash/.my_fun.out"
REF_ERR_OUT="trash/.original_err.out"
TEST_ERR_OUT="trash/.my_fun_err.out"
DIFF_OUT="trash/.diff.out"
DIFF_ERR="trash/.diff_err.out"

G="\033[0;92m"
R="\033[0;91m"
D="\033[00m"

TARGET=target/release/./alignement

#builds the optimized executable if it doesnt exist
if [ ! -e "$TARGET" ];then
    cargo build --release
fi

#to run the program
#->"$TARGET"


init()
{
    mkdir trash
}

#used to test the difference between our binary and a cpp implementation binary
#USAGE : tes {file1} {file2} ...
tes() {
    "$TARGET" "$@" > "$REF_OUT" 2> "$REF_ERR_OUT"
    "A CPP BINARY IMPLEMENTING THE ALGO $@" > "$TEST_OUT" 2> "$TEST_ERR_OUT"
    diff -u --color="always" "$REF_OUT" "$TEST_OUT" > "$DIFF_OUT" 2>&1
    if [ $? -eq 0 ]; then
        diff -u --color="always" "$REF_ERR_OUT" "$TEST_ERR_OUT" > "$DIFF_ERR" 2>&1
        if [ $? -eq 0 ]; then
            echo "$G[OK]$D $@"
        else
            echo "$R[KO]$D"
            echo "$@"
            cat "$DIFF_ERR"
        fi
    else
            echo "$R[KO]$D"
            echo "given input : $@"
            cat "$DIFF_OUT"
    fi
}
basic()
{
    echo "========== BASICS TESTS BEGIN =========="
    tes testfiles/file1 testfiles/file2 
    tes testfiles/more1 testfiles/more2 
    echo "========== BASICS TESTS END =========="
}
testsuite()
{
    basic
}
init
testsuite
rm $REF_OUT
rm $TEST_OUT
rm -rf trash 
