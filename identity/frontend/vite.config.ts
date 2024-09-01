import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    proxy: {
      "/plugin.json": {
        target: "http://127.0.0.1:8090",
      },
      "/extern/daemon": {
        target: "http://127.0.0.1:8090",
      },
    },
  },
});
