import { defineConfig } from "vite";
import solidPlugin from "vite-plugin-solid";

export default defineConfig({
  plugins: [solidPlugin()],
  base: "./",
  server: {
    port: 3003,
  },
  build: {
    target: "esnext",
  },
});
