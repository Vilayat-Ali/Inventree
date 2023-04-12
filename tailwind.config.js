/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: 'jit',
  purge: {
      mode: "all",
      content: [
          "./src/frontend/src/**/*.rs",
          "./src/frontend/index.html",
          "./src/frontend/**/*.html",
          "./src/frontend/**/*.css",
      ],
  },
  theme: {},
  variants: {},
  daisyui: {
    themes: ["light", "dark", "cupcake", "bumblebee", "emerald", "corporate", "synthwave", "retro", "cyberpunk", "valentine", "halloween", "garden", "forest", "aqua", "lofi", "pastel", "fantasy", "wireframe", "black", "luxury", "dracula", "cmyk", "autumn", "business", "acid", "lemonade", "night", "coffee", "winter"],
  },
  plugins: [require("daisyui")],
};

