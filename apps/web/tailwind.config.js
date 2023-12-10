const baseConfig = require('@dyn/ui/tailwind.config');

/** @type {import('tailwindcss').Config} */
module.exports = {
	content: ['./src/components/**/*.{ts,tsx}', './src/app/**/*.{ts,tsx}'],
	presets: [baseConfig]
};
