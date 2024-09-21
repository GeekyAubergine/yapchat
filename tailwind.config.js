module.exports = {
  darkMode: "class",
  content: ["./templates/**/*.html"],
  safelist: [],
  theme: {
    extend: {
      colors: {
        background: "#181818",
        headings: {
          DEFAULT: "#080808",
          dark: "#EDEDED",
        },
        text: "#EDEDED",
        secondary: "#ABA9B0",
        accent: "#ED95E6",
        border: "#404040",
        middleGray: "#181818",
      },
    },
    fontFamily: {
      sans: ["Helvetica", "Arial", "sans-serif"],
    },
  },
  plugins: [require("@tailwindcss/nesting")(require("postcss-nesting"))],
};
