const defaultTheme = require("tailwindcss/defaultTheme");
const colors = require("tailwindcss/colors");

module.exports = {
  darkMode: 'media',
  content: [
    "./src/**/*.{rs,html,css}",
    "./index.html",
    "./static/styles/**/*.css",
  ],
  theme: {
    fontFamily: {
        heading: ["Comfortaa", "sans"],
        ...defaultTheme.fontFamily,
    },
    screens: {
        "2xs": "370px",
        xs: "475px",
        ...defaultTheme.screens,
    },
    colors: {
        // Full color palette
        transparent: "transparent",
        current: "currentColor",
        black: colors.black,
        white: colors.white,
        gray: colors.gray,
        red: colors.red,
        orange: colors.orange,
        amber: colors.amber,
        yellow: colors.amber,
        green: colors.emerald,
        indigo: colors.indigo,
    },
},
variants: {},
plugins: [require("tailwind-hamburgers")],
};