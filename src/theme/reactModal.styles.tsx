import { css } from 'styled-components'

export const reactModalStyles = css`
	.modal-overlay {
		z-index: 100;
		top: 0;
		left: 0;
		position: fixed !important;
		background: ${props => props.theme.colors.black.tint(0).val};
		transition: all ${props => props.theme.times.tranM};
		width: 100vw;
		height: 100vh;
		display: flex;
		align-items: center;
		justify-content: center;

		&.pic {
			background: ${props => props.theme.colors.black.tint(80).val};
		}
		&.ReactModal__Overlay--after-open {
			background: ${props => props.theme.colors.black.tint(60).val};
			.modal-box {
				/* transform: translate3d(0, 0, 0); */
				/* NOTE: We can't use transform with react-beautiful-dnd */
				opacity: 1;
			}
		}
		&.ReactModal__Overlay--before-close {
			background: ${props => props.theme.colors.black.tint(0).val};
			.modal-box {
				opacity: 0;
				/* transform: translate3d(0, 50px, 0); */
				/* NOTE: We can't use transform with react-beautiful-dnd */
			}
		}
	}

	.modal-box {
		width: auto;
		height: auto;
		transition: all ${props => props.theme.times.tranS};
		/* NOTE: We can't use transform with react-beautiful-dnd */
		/* transform: translate3d(0, 50px, 0); */
	}
`
