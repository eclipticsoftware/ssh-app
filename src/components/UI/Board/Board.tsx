import { ReactNode } from 'react'
import styled, { css } from 'styled-components'
import { Icon } from '../Icon'

export const boardStyles = css`
	background: ${props => props.theme.colors.white.val};
	box-shadow: 0 2px 3px ${props => props.theme.colors.grey.opacity(70).val};
	border-radius: 10px;
	overflow: hidden;
	display: flex;
	flex-direction: column;
	max-width: 600px;

	& > header {
		padding: 1.5em 4em;
		background: ${props => props.theme.colors.medGrey.val};
		color: ${props => props.theme.colors.white.val};
		display: flex;
		align-items: center;

		.icon {
			height: 2rem;
			width: auto;
			margin-right: 1em;
		}

		h2 {
			padding: none;
			margin: none;
			font-size: 1.5rem;
		}
	}

	& > main {
		padding: 3em 4em;
		flex-grow: 1;
	}
`

const BoardView = styled.div`
	${boardStyles}
`

export type BoardProps = {
	boardTitle: string
	children: ReactNode
	className?: string
}
export const Board = ({ boardTitle, children, className }: BoardProps): JSX.Element => {
	return (
		<BoardView className={`board${className ? ` ${className}` : ''}`}>
			<header>
				<Icon type='ssh' padRight />
				<h2>{boardTitle}</h2>
			</header>
			<main>{children}</main>
		</BoardView>
	)
}
