cargo build --release
cargo build --target x86_64-pc-windows-gnu --release
mkdir Geometry-Rays
mv ./target/x86_64-pc-windows-gnu/release/geometry-rays.exe ./Geometry-Rays
mv ./target/release/geometry-rays ./Geometry-Rays
cp -r ./Resources ./Geometry-Rays
cp -r ./save-data ./Geometry-Rays
7z a -tzip Geometry-Rays.zip Geometry-Rays
rm -rf ./Geometry-Rays
