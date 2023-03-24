serve-web:
    cargo watch -- just build-web & serve wasm && fg

build-web:
    cargo make --profile release build-web

    rm -r wasm
    mkdir -p wasm
    cp -r assets/ wasm/assets
    cp target/wasm_bg.wasm wasm/
    cp target/wasm.js wasm/
    cp index.html wasm/

    zip -r dist.zip wasm
