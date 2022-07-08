import { Theme } from '@eclo/mode'
import { EcloColor } from '@eclo/mode/dist/lib/EcloColor'
import 'styled-components'
import colors from './theme/colors'

export interface ProjectTheme extends Theme {
	colors: Theme['colors'] & {
		[Property in keyof typeof colors]: EcloColor
	}
}

// and extend them!
declare module 'styled-components' {
	export interface DefaultTheme extends ProjectTheme {}
}
