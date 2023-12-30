# NOTE: add code to path/to/cargo/config
#
# [build]
# target = ["YOUR_TARGET"]

build() {
    wasm-pack build ./yiiy \
        --target web \
        --release \
        -d ../www/wasm \
        --out-name yiiy.wasm
}

build-ts() {
    cd yiiy-web
    tsc
}

server(){
    dufs --render-index \
        --allow-search \
        ../www
        #--enable-cors \
}

build && build-ts && server
