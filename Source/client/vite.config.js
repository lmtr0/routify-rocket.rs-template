import { svelte } from '@sveltejs/vite-plugin-svelte';
import { defineConfig } from 'vite';

const watch = process.env.WATCH_BUILD == "true"

export default defineConfig({
    mode: 'production',
    build: {
        minify: true,
        brotliSize: true,
        outDir: "../../public",
        watch,
        sourcemap: false,
    },
    
    plugins: [svelte()],
});
