import { ReactNode } from 'react'
import styled, { css } from 'styled-components'

export const boardStyles = css`
	background: ${props => props.theme.colors.white.val};
	box-shadow: 0 2px 3px ${props => props.theme.colors.grey.opacity(70).val};
	border-radius: 10px;
	overflow: hidden;
	display: flex;
	flex-direction: column;
	width: 650px;
	height: 650px;

	& > header {
		padding: 1.5em 4em;
		background: ${props => props.theme.colors.medGrey.val};
		color: ${props => props.theme.colors.white.val};
		display: flex;
		align-items: center;
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
	boardHeader: ReactNode
	children: ReactNode
	className?: string
}
export const Board = ({ boardHeader, children, className }: BoardProps): JSX.Element => {
	return (
		<BoardView className={`board${className ? ` ${className}` : ''}`}>
			<header>{boardHeader}</header>
			<main>{children}</main>
		</BoardView>
	)
}
