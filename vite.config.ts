import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import process from "node:process";
import { fileURLToPath } from "node:url";

const colors = fileURLToPath(
  new URL("./src/Styles/colors.scss", import.meta.url)
).replace(/\\/g, "/");

const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig(() => ({
  plugins: [vue()],

  css: {
    preprocessorOptions: {
      scss: {
        loadPaths: ["src/Styles"],
        additionalData: (content: string, filename: string) =>
          filename.split("?")[0].endsWith(".vue")
            ? `@use "colors" as *;\n${content}`
            : content,
      },
    },
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));