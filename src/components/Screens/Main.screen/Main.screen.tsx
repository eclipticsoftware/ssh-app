import styled, { css } from 'styled-components'
import { useStore } from '../../Store/Store.provider'
import { Board } from '../../UI/Board'
import { BoardHeader } from '../../UI/Board.header/Board.header'
import { ConnectScreen } from '../Connect.screen'
import { ConnectedScreen } from '../Connected.screen'

export const mainScreenStyles = css`
	width: 100%;
	height: 100%;
	min-width: 100vw;
	min-height: 100vh;
	display: flex;
	align-items: center;
	justify-content: center;
	background: ${props => props.theme.colors.lightGrey.val};

	hr {
		margin: 1em 0;
	}
`

const MainScreenView = styled.div`
	${mainScreenStyles}
`

export const MainScreen = (): JSX.Element => {
	const { status } = useStore()
	const showConnectedScreen = status === 'CONNECTED' || status === 'RETRYING'

	return (
		<MainScreenView>
			<Board boardHeader={<BoardHeader />}>
				{showConnectedScreen ? <ConnectedScreen /> : <ConnectScreen />}
			</Board>
		</MainScreenView>
	)
}
