class Plugin {
    async prompt(message) {
        return [
            {
                type: 'text',
                value: `I was configured with: ${this.deep}, and was sent: ${message.value}`
            }
        ];
    }

    configure(configuration) {
        this.deep = configuration.deep;
    }
}

export const plugin = new Plugin();