import * as esbuild from 'esbuild';
import {DefaultNodePolyfill, polyfill} from "./src";
import {polyfillNode} from 'esbuild-plugin-polyfill-node';

// Build polyfills
await esbuild.build({
    entryPoints: [
        'src/polyfills/empty.ts',
        'src/polyfills/global.ts',
        'src/polyfills/stream.ts',
        'src/polyfills/web-stream.ts'
    ],
    bundle: true,
    minify: false,
    treeShaking: true,
    format: "esm",
    target: "es2022",
    platform: "node",
    outdir: 'dist',
    external: [
        '@aws-sdk/credential-provider-http'
    ],
    plugins: [
        polyfillNode({...DefaultNodePolyfill}),
        polyfill({
            'process/': {
                shimFile: './src/polyfills/process.js'
            }
        })
    ],

});

await esbuild.build({
    entryPoints: ['main.ts'],
    outfile: 'out.js',
    bundle: true,
    minify: false,
    treeShaking: true,
    format: "esm",
    target: "es2022",
    platform: "node",
    external: [
        '@aws-sdk/credential-provider-http'
    ],
    plugins: [
        polyfillNode({...DefaultNodePolyfill}),
        polyfill({
            'stream': {
                shimFile:
                    './dist/stream.js',
            }
        })
    ]
});