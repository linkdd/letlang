/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./content/**/*.md', './templates/**/*.html'],
  theme: {
    extend: {
    },
  },
  safelist: [
    '.grammkit-diagram',
    '.grammkit-diagram-title',
    '.grammkit-diagram-svg',
    {
      pattern: /table-col\d+-.*/,
    },
  ],
  plugins: [],
}

