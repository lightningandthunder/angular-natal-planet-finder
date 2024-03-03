#!/bin/bash
version=$(grep -oP 'version = "\K[^"]+' Cargo.toml)

windows_archive="angular-natal-planet-finder-windows-$version"

cargo clean
[ -d dist ] && rm -r dist
[ -d "$windows_archive" ] && rm -r $windows_archive
[ -f "$windows_archive".zip ] && rm "$windows_archive".zip
mkdir -p target/x86_64-pc-windows-gnu
mkdir dist
cp src/swe/dll/swedll64.dll target/x86_64-pc-windows-gnu/
cross build --target x86_64-pc-windows-gnu --release

# windows
mkdir $windows_archive
mkdir "$windows_archive"/output
cp src/swe $windows_archive -r
rm -r "$windows_archive"/swe/dll
cp src/swe/dll/swedll64.dll $windows_archive
cp target/x86_64-pc-windows-gnu/release/angular-natal-planet-finder.exe $windows_archive
zip -r "$windows_archive".zip $windows_archive
mv "$windows_archive".zip dist
[ -d "$windows_archive" ] && rm -r $windows_archive