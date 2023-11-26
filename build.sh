set -e

cd `dirname $0`
cd nelsie-builder
cargo build --release
cp -r target/release/nelsie-builder ../nelsie-api/nelsie/backend
