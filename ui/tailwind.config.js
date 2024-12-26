const colors = require('tailwindcss/colors')

module.exports = {
    content: ["./src/**/*.{html,tsx,ts}"],
    theme: {
        extend: {
            colors: {
                danger: colors.red,
                primary: colors.blue
            }
        },
    },
    plugins: [],
}