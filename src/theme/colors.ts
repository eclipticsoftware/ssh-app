const colors = {
	primary: '#5ABDDC',
	secondary: '#ec9800',

	black: '#0F0F0E',
	white: '#FFF',
	whiteTint: '#ffffffd9',
	blue: '#0569b1',
	medBlue: '#138de4',
	midnightBlue: '#1D253A',
	greyTint: '#0f0f0e1a',
	lightGrey: '#c6cacc',
	grey: '#989ea0',
	medGrey: '#758B9A',
	darkGrey: '#565656',
	midnightGrey: '#3c3c3c',
	yellow: '#EADB1F',
	gold: '#eaad00',
	darkGold: '#bb7900',
	darkGold50: 'rgba(187,121,0,0.5)',
	red: '#dc3300',
	green: '#5bbb12',
	orange: '#ec9800',
}
export default colors

export type ThemeColor = keyof typeof colors
