/*
 =================================================
  Swf-Theme Config and Init
  See: https://github.com/eclipticsoftware/mode
 =================================================
* */

import { createTheme, ThemeOptions } from '@eclo/mode'
import { canPrint } from '../app.config'
import { ProjectTheme } from '../theme'
import colors from './colors'

export const themeConfig: ThemeOptions = {
	colors,

	fonts: {
		textFamily: 'PT Sans Caption, sans-serif',
		titleFamily: 'Oswald, sans-serif',
	},

	// breaks: {
	// 	ldesk: {
	// 		num: 1580,
	// 		unit: 'px',
	// 	},
	// },

	sizes: {
		gutter: {
			mobile: {
				num: 20,
				unit: 'px',
			},
			tablet: {
				num: 5,
				unit: 'vw',
			},
			sdesk: {
				num: 10,
				unit: 'vw',
			},
			ldesk: {
				num: 10,
				unit: 'vw',
			},
		},
		header: {
			mobile: {
				num: 70,
				unit: 'px',
			},
			tablet: {
				num: 50,
				unit: 'px',
			},
			sdesk: {
				num: 70,
				unit: 'px',
			},
		},
		deskNav: {
			num: 200,
			val: '200px',
		},
		alertbar: {
			num: 30,
			val: '30px',
		},
	},

	printLogs: canPrint,
}

export const theme = createTheme(themeConfig) as ProjectTheme
