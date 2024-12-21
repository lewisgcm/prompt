class Plugin {
    async test() {
        throw "this is a error";
    }
}

export const plugin = new Plugin();