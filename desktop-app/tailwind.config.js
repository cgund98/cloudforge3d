/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/index.html", "./src/**/*.{html,js,svelte,ts}"],
  theme: {
    extend: {},
  },
  safelist: [
    "badge-info",
    "badge-warning",
    "badge-success",
    "badge-error",
    "bg-info",
    "bg-warning",
    "bg-success",
    "bg-error",
    "text-info",
    "text-warning",
    "text-success",
    "text-error",
  ],
  daisyui: {
    themes: [
      "light",
      "dark",
      "cupcake",
      "bumblebee",
      "emerald",
      "corporate",
      "synthwave",
      "retro",
      "cyberpunk",
      "valentine",
      "halloween",
    ],
  },
  plugins: [require("@tailwindcss/typography"), require("daisyui")],
};
