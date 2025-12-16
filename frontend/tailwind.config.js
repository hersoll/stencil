export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        bg_dark: 'rgb(var(--bg-dark) / <alpha-value>)',
        bg: 'rgb(var(--bg) / <alpha-value>)',
        bg_light: 'rgb(var(--bg-light) / <alpha-value>)',
        text_color: 'rgb(var(--text) / <alpha-value>)',
        text_muted: 'rgb(var(--text-muted) / <alpha-value>)'
      }
    }
  }
};
