import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";
import tailwindcss from '@tailwindcss/vite'

export default defineConfig({
  plugins: [
    tailwindcss(),
    wasm(),
  ],
  base: '/pouet-unit'
});