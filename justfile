build-web:
    cargo make --profile release build-web

    mkdir -p wasm
    cp target/wasm_bg.wasm wasm/
    cp target/wasm.js wasm/
    cp index.html wasm/

    zip -r dist.zip wasm