build() {
    wasm-pack build ./ \
        --target web \
        --release \
        -d ./www/wasm \
        --out-name web.wasm
}

# server(){
#     dufs --render-index \
#         --allow-search \
#         ./www
#         #--enable-cors \
# }
#
#build && server

build
