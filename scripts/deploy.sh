wasm-pack build --target no-modules --no-typescript --release
mkdir deploy
HASH=$(git rev-parse --short HEAD)
mkdir deploy/$HASH
cp pkg/mondrians.js pkg/mondrians_bg.wasm deploy/$HASH
cp static/* deploy/$HASH
mv deploy/$HASH/index.html deploy/
# Replace {hash} in index.html with the hash of the current commit
sed -i "s/{hash}/$HASH/g" deploy/index.html
