import { css } from 'styled-components'

export type SpinnerStylesProps = {
	height?: string
}

export const spinnerStyles = css<SpinnerStylesProps>`
	position: relative;
	display: flex;
	align-items: center;
	justify-content: center;
	background: ${props => props.theme.colors.offWhite?.val};
	height: ${props => props.height || 'auto'};
	transition: all ${props => props.theme.times?.tranM};

	&.__no-bg {
		background: none;
	}

	&.__invert {
		background: ${props => props.theme.colors.grey?.val};
		&.__no-bg {
			background: none;
		}
	}
	&.__overlay {
		position: absolute;
		top: 0;
		left: 0;
		height: 100%;
		width: 100%;
		z-index: 100;
	}

	&.__sm {
	}
`
