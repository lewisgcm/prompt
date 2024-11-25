export function tool({sign}) {
    return {song: "Elemental Hotel", artist: "8 Storey Hike", sign: sign};
}

export const schema = {
    description: 'Get the most popular song played on a radio station.',
    arguments: {
        sign: {
            type: 'string',
            description: 'The call sign for the radio station for which you want the most popular song. Example calls signs are WZPZ and WKRP.',
            required: true,
        }
    }
};