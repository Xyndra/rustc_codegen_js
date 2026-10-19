const typography = process.env.TAILWIND_TYPOGRAPHY !== "false"
  ? require("@tailwindcss/typography")
  : null;
const forms = process.env.TAILWIND_FORMS !== "false"
  ? require("@tailwindcss/forms")
  : null;

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [],
  theme: { extend: {} },
  plugins: [typography, forms].filter(Boolean)
};
