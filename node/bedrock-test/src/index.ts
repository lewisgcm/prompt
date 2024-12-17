import * as esbuild from "esbuild";
import {resolve} from "node:path";
import {PolyfillNodeOptions} from 'esbuild-plugin-polyfill-node';

type PolyFillConfig = { shimFile: string }

export const DefaultNodePolyfill = Object.freeze({
    globals: {
        global: true,
        buffer: false,
        process: false,
    },
    polyfills: {
        "assert/strict": false,
        buffer: false,
        child_process: false,
        console: false,
        constants: false,
        crypto: false,
        domain: false,
        events: false,
        fs: false,
        "fs/promises": false,
        http: 'empty',
        https: 'empty',
        module: false,
        net: false,
        os: false,
        path: false,
        perf_hooks: false,
        process: false,
        punycode: false,
        querystring: false,
        readline: false,
        stream: false,
        string_decoder: true,
        sys: false,
        timers: false,
        "timers/promises": false,
        tty: false,
        url: false,
        util: true,
        vm: false,
        worker_threads: false,
        zlib: false,
    }
} as PolyfillNodeOptions);

export function polyfill(moduleNames: { [key: string]: PolyFillConfig }): esbuild.Plugin {
    const filter = new RegExp(`^(node:)?(${Object.keys(moduleNames).join("|")})$`);

    return {
        name: "polyfills",

        async setup(build) {
            build.onResolve({filter}, async ({path}) => {
                const [, , moduleName] = path.match(filter)!;

                const polyfill = moduleNames[moduleName];

                if (!polyfill) {
                    return;
                } else {
                    return {path: resolve(polyfill.shimFile)};
                }
            });

            resolve("./dist/global.js");
        },
    };
}