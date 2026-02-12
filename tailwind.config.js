/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        // Warm, soft color palette
        primary: {
          DEFAULT: '#FF8B7B',
          light: '#FFB88C',
          dark: '#E07A5F',
        },
        background: {
          DEFAULT: '#FAFAFA',
          warm: '#F8F6F4',
          card: '#FFFFFF',
        },
        neutral: {
          light: '#F5E6D3',
          DEFAULT: '#E8DCC8',
          dark: '#4A4A4A',
        },
        accent: {
          gold: '#FFD966',
          terracotta: '#E07A5F',
          peach: '#FFF5E6',
        },
        status: {
          todo: '#9CA3AF',
          doing: '#60A5FA',
          done: '#34D399',
          overdue: '#EF4444',
        },
      },
      borderRadius: {
        DEFAULT: '0.5rem',
        lg: '0.75rem',
        xl: '1rem',
      },
      boxShadow: {
        soft: '0 4px 12px rgba(0, 0, 0, 0.08)',
        warm: '0 2px 8px rgba(224, 122, 95, 0.15)',
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
      },
    },
  },
  plugins: [],
}
