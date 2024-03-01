import { skeleton } from '@skeletonlabs/tw-plugin';
import { join } from 'path';
import type { Config } from 'tailwindcss';
import { cambiaTheme } from './cambia-theme';

const config = {
	darkMode: 'class',
	content: [
		'./src/**/*.{html,js,svelte,ts}',
		join(require.resolve(
			'@skeletonlabs/skeleton'),
			'../**/*.{html,js,svelte,ts}'
		),
	],
	safelist: [
		{ pattern: /bg-(success|error|warning|surface)-(300|700)/ }
	],
	theme: {
		extend: {
			zIndex: {
				'max': '9999999999',
			},
			fontSize: {
				'xxs': '0.5rem',
			},
		},
		animation: {
			blob: "blob 7s infinite",
		},
		keyframes: {
			blob: {
				"0%": {
					transform: "translate(0px, 0px) scale(1)",
				},
				"33%": {
					transform: "translate(40px, -5px) scale(1.1)",
				},
				"66%": {
					transform: "translate(-30px, 20px) scale(0.9)",
				},
				"100%": {
					transform: "tranlate(0px, 0px) scale(1)",
				},
			},
		},
	},
	plugins: [require('@tailwindcss/forms'),require('@tailwindcss/typography'), skeleton({themes: {custom: [ cambiaTheme ]}})],
} satisfies Config;

export default config;
