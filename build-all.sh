#!/bin/bash
echo building...
cargo build -r
echo building windows...
cargo build -r --target x86_64-pc-windows-gnu

cp ./target/release/term-cave-crawl-rpg ./executables/
cp ./target/x86_64-pc-windows-gnu/release/term-cave-crawl-rpg.exe ./executables/
echo ==[Done!]==

