export function pluck<T>(object: { [key: string]: T }, ...keys: string[]): { [key: string]: T } {
    return Object.entries(object).filter(([key, value]) => keys.includes(key)).reduce((acc, [key, value]) => {
        return {
            ...acc,
            [key]: value
        }
    }, {});
}