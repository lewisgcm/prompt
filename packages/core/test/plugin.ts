export async function tool({input}: any): Promise<any> {
    return {testOutput: `Some output ${input}`};
}

export const schema = {
    description: 'This is a test plugin',
    arguments: {
        input: {
            type: 'string',
            description: 'Test argument',
            required: true,
        }
    }
};