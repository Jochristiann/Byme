/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        "./index.html",
        "./src/**/**/*.{js,ts,jsx,tsx}",
    ],
    theme: {
        extend: {
            colors: {
                primary: '#0E2148',
                secondary: '#483AA0',
                tertiary: '#7965C1',
                test: '#000000',
            },
        },
    },
    plugins: [],
};