/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      fontFamily: {
        mono: ['JetBrains Mono', 'ui-monospace', 'SFMono-Regular', 'monospace'],
        sans: ['Inter', 'ui-sans-serif', 'system-ui', 'sans-serif'],
      },
      colors: {
        oae: {
          bg: "#0a0a0b",
          panel: "#141416",
          border: "#232326",
          muted: "#9f9fa9",
          accent: "#e4e4e7",
        }
      }
    },
  },
  plugins: [],
}
