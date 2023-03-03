build-web:
    cargo make --profile release build-web

    mkdir -p wasm
    cp -r assets/ wasm/assets
    cp target/wasm_bg.wasm wasm/
    cp target/wasm.js wasm/
    cp index.html wasm/

    zip -r dist.zip wasm