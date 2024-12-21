class Plugin {
    configuration() {
        return [
            {
                input: (context) => ({
                    region: {
                        displayName: 'Region',
                        type: 'select',
                        required: true,
                        options: ['us-east-1', 'us-east-2', 'eu-west-1']
                    }
                })
            },
            {
                input: (context) => ({
                    model: {
                        displayName: 'Other selected based on previous region selection',
                        type: 'select',
                        required: true,
                        options: [`${context.region}-option-1`, `${context.region}-option-2`, `${context.region}-option-3`]
                    }
                })
            }
        ];
    }
}

export const plugin = new Plugin();