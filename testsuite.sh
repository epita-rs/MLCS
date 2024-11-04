TARGET=target/release/./alignement
if [ ! -e "$TARGET" ];then
    cargo build --release
fi
#runs the program
"$TARGET"
