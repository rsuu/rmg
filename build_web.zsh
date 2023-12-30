# NOTE: add code to path/to/cargo/config
#
# [build]
# target = ["YOUR_TARGET"]

build() {
    wasm-pack build ./ \
        --target web \
        --release \
        -d ./www/wasm \
        --out-name rmg.wasm \
        -- --no-default-features -F "web,ex_tar,ex_zip"

    rm ./www/wasm/.gitignore
}

build-ts() {
    cd yiiy-web
    tsc
}

server(){
    dufs --render-index \
        --allow-search \
        ./www
        #--enable-cors \
}

build && server
