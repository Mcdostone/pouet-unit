build:
     wasm-pack build --target web --out-dir frontend/library
     rm frontend/library/.gitignore

dev:
     npm --prefix frontend run dev