import { defineConfig } from "vite";
import preact from "@preact/preset-vite";

const base = process.env.GITHUB_PAGES === "true" ? "/rust-tetris/" : "/";

export default defineConfig({
  base,
  plugins: [preact()],
  worker: {
    format: "es",
  },
});
