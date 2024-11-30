// https://api.api-ninjas.com/v1/facts

export async function tool({sign}) {
    const response = await fetch("asdasdasd");
    const json = await response.json();

    return {fact: json[0].fact};
}

export const schema = {
    description: 'Get a random blurb for the day',
    arguments: {
    }
};