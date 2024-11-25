/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*.html", "./src/**/*.rs"],
  theme: {
    extend: {},
  },
  plugins: [require("daisyui")],
  daisyui: {
    themes:  [
      {
        gctheme: {
          "primary": "#003366",   
          "secondary": "#ffcc00",
          "accent": "#ffb84d",
          "neutral": "#d1d1d1",
          "base-100": "#FFFFFF",
          "info": "#00bfff",
          "success": "#4caf50",
          "warning": "#ff9800",
          "error": "#d32f2f",
        },
      },
    ],
},
}
