build:
     wasm-pack build --target web --out-dir frontend/library
     rm frontend/library/.gitignore
     npm --prefix frontend run build

dev:
     npm --prefix frontend run dev