/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./**/*.html"],
  theme: {
    extend: {
      colors: {
        'background': '#171821',
        'txt': '#D2D2D2',
        'txthl': '#00ADB5',
        'accepted': '#00B548',
        'rejected': '#B50B00',
        'pending': '#B58D00',
        'foreground1': '#222330',
        'foreground2': '#2F3041',
      },
    },
  },
  plugins: [],
};

