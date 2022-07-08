import styled, {css} from 'styled-components'
import {Icon} from '../Icon'

export const errorBlockStyles = css`
	color: ${p => p.theme.colors.err.val};

	.err-box {
		display: flex;
		align-items: center;
		margin-bottom: 1rem;
	}

	.icon,
	.err-msg {
		color: ${props => props.theme.colors.err.val};
	}

	.icon {
		height: 1.4em;
		width: auto;
	}

	.icon-box {
	}

	.err-msg {
	}

	&._display-inline {
		width: auto;
		.err-box {
			height: auto;
			width: auto;
			padding: 0;
			border: solid 1px;
		}
		.icon-box {
			display: flex;
			align-items: center;
			justify-content: center;
			padding: 0.5em;
			background: ${p => p.theme.colors.err.val};
			.icon {
				color: ${p => p.theme.colors.white.val};
				height: 1.5rem;
				width: auto;
			}
		}
		.err-msg {
			padding: 0.5rem;
		}
	}

	&._display-block {
		.err-box {
			align-items: flex-start;
			border: solid 4px;
		}
		.icon-box {
			padding: 1rem;
			display: flex;
			align-items: center;
			justify-content: center;
			.icon {
				height: 5rem;
			}
		}
		.err-msg {
			padding: 1rem;
		}
	}

	&._display-full {
	}
`

const ErrorBlockView = styled.div`
	${errorBlockStyles}
`


export type ErrorBlockErr = string | null

type ErrorBlockProps = {
	error?: ErrorBlockErr
	className?: string
	display?: 'inline' | 'full' | 'block'
}
export const ErrorBlock = ({
	error,
	className,
	display = 'inline',
}: ErrorBlockProps): JSX.Element => {

	return error ? (
		<ErrorBlockView
			className={`error-block _display-${display}${className ? ` ${className}` : ''}`}
		>

					<div className='err-box'>
						<div className='icon-box'>
							<Icon type='err' />
						</div>
						<p className='err-msg' color='err'>
							{error}
						</p>
					</div>
				

		</ErrorBlockView>
	) : (
		<></>
	)
}
