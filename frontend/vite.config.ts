import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [react()],
  clearScreen: false,
  build: {
    rollupOptions: {
      input: {
        main: "index.html",
        qaVisual: "qa-visual.html",
      },
    },
  },
  server: {
    port: 5173,
    strictPort: true,
  },
});
