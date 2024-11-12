#!/bin/sh

REF_OUT="trash/.original_fun.out"
TEST_OUT="trash/.my_fun.out"
REF_ERR_OUT="trash/.original_err.out"
TEST_ERR_OUT="trash/.my_fun_err.out"
DIFF_OUT="trash/.diff.out"
DIFF_ERR="trash/.diff_err.out"
CPP="testfiles/cpp-implementation/./mlcs"

G="\033[0;92m"
R="\033[0;91m"
D="\033[00m"

# catching the executable name in the Cargo.toml file
# CONTEXT : using sed with options extended regex + silent mode
# then encapsulating the name in a group 
# then output the captured group with option p
BIN=$(sed -En 's/name = "([^"]+)"/\1/p' Cargo.toml)

TARGET=target/release/./"$BIN"

#builds the rust executable if it doesnt exist
if [ ! -e "$TARGET" ];then
    cargo build --release
fi

#builds the cpp executable if it doesnt exist
if [ ! -e "$CPP" ];then
    cd testfiles/cpp-implementation/
    make
    cd ../..
fi
# to run the program
# "$TARGET"


init()
{
    mkdir trash
}

# used to test the difference between the binary to test
# and a cpp binary
#USAGE : tes {file1} {file2} ...
tes() {
    "$TARGET" "$@" > "$REF_OUT" 2> "$REF_ERR_OUT"
    "$CPP" "$@" > "$TEST_OUT" 2> "$TEST_ERR_OUT"
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
    tes testfiles/big1 testfiles/big2 
    tes testfiles/big3 testfiles/big2 
    tes testfiles/big3 testfiles/more2 testfiles/big1 testfiles/big4 
    tes testfiles/big1 testfiles/big2 testfiles/big3 testfiles/big4
    echo "========== BASICS TESTS END =========="
} 
hardcore()
{
    tes testfiles/toobig1 testfiles/toobig2 
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
