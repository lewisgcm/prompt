import * as esbuild from 'esbuild';
import esbuildPluginTsc from 'esbuild-plugin-tsc';
import path from "path";

const outFile = path.join(__dirname, 'dist', 'prompt.js');

esbuild.build({
    minify: true,
    entryPoints: ['src/main.ts'],
    outfile: outFile,
    platform: 'node',
    bundle: true,
    plugins: [
        esbuildPluginTsc({
            force: true
        }),
    ]
});