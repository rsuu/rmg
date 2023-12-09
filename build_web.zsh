# TODO: :)
wasm-pack build src \
    --target web \
    -d ./web \
    --out-name rmg.wasm \
    -- --features web --no-default-features
