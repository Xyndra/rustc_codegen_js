const base = require("./tailwind.config.js");

/**
 * Complete build: every utility class Tailwind can generate, including the
 * class sets contributed by the plugins. The per-page critical CSS is cut out
 * of this file by `fw_tailwind_build` (lightningcss) at site build time.
 *
 * @type {import('tailwindcss').Config}
 */
module.exports = {
  ...base,
  content: [],
  safelist: [{ pattern: /.*/ }]
};
