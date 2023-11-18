const plugin = require('tailwindcss/plugin');

/** @type {import('tailwindcss').Config} */
export default {
    content: ['./index.html', './src/**/*.{js,ts,jsx,tsx}'],
    theme: {
        fontFamily: {
            sans: ['"Inter"', 'sans-serif'],
        },
        extend: {
            colors: {
                primary: '#14100E',
                secondary: '#A1887F',
                accent: '#FF8A65',
                dimmed: '#8D6E63',
            },
            boxShadow: {
                base: '0px 0px 15px 0px rgba(0,0,0,0.1)',
            },
            textShadow: {
                sm: '0 1px 2px var(--tw-shadow-color)',
                DEFAULT: '0 2px 4px var(--tw-shadow-color)',
                lg: '0 8px 16px var(--tw-shadow-color)',
            },
        },
    },
    plugins: [
        plugin(function ({ matchUtilities, theme }) {
            matchUtilities(
                {
                    'text-shadow': (value) => ({
                        textShadow: value,
                    }),
                },
                { values: theme('textShadow') },
            );
        }),
    ],
};
