import * as esbuild from 'esbuild';
import {DefaultNodePolyfill, polyfill} from "./src";
import {polyfillNode} from 'esbuild-plugin-polyfill-node';

const isProduction = process.env.NODE_ENV == 'production';

// Build polyfills
// @ts-ignore
await esbuild.build({
    entryPoints: [
        'src/polyfills/empty.ts',
        'src/polyfills/global.ts',
        'src/polyfills/stream.ts',
        'src/polyfills/web-stream.ts'
    ],
    bundle: true,
    minify: isProduction,
    treeShaking: true,
    format: "esm",
    target: "es2022",
    platform: "node",
    outdir: 'dist/polyfills',
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

// @ts-ignore
await esbuild.build({
    entryPoints: ['bedrock.ts'],
    outdir: 'dist',
    bundle: true,
    minify: isProduction,
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
                    './dist/polyfills/stream.js',
            }
        })
    ]
});