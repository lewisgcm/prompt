import esbuildPluginTsc from 'esbuild-plugin-tsc';

function createBuildSettings(options) {
    return {
        platform: 'node',
        entryPoints: ['src/main.ts'],
        outfile: 'dist/prompt.cjs',
        bundle: true,
        plugins: [
            esbuildPluginTsc({
                force: true
            }),
        ],
        ...options
    };
}

import * as esbuild from 'esbuild';

const settings = createBuildSettings({minify: true});

await esbuild.build(settings);