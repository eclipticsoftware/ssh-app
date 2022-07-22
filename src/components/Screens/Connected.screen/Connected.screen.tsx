import { invoke } from '@tauri-apps/api'
import styled, { css } from 'styled-components'
import { constants } from '../../../app.config'
import { ErrorBlock } from '../../UI/ErrorBlock'
import { ConnectionStatus } from '../Main.screen'

export const connectedScreenStyles = css`
	.disconnect-btn {
		outline: none;
		box-shadow: none;
		border: solid 2px;
		padding: 1em 2em;
		color: ${props => props.theme.colors.secondary.val};

		&:hover {
			color: ${props => props.theme.colors.white.val};
			background: ${props => props.theme.colors.secondary.val};
		}

		&:disabled {
			color: ${props => props.theme.colors.lightGrey.val} !important;
			background: none !important;
		}
	}
`

const ConnectedScreenView = styled.div`
	${connectedScreenStyles}
`

export type ConnectedScreenProps = {
	status: ConnectionStatus
	onDisconnect: () => void
	systemErr: string | null
}

export const ConnectedScreen = ({
	status,
	systemErr,
	onDisconnect,
}: ConnectedScreenProps): JSX.Element => {
	const disconnectHandler = async () => {
		try {
			await invoke(constants.endTunnel)
			onDisconnect()
		} catch {}
	}
	return (
		<ConnectedScreenView>
			{systemErr ? <ErrorBlock error={systemErr} /> : null}
			<button
				className='disconnect-btn'
				onClick={disconnectHandler}
				disabled={status === 'RETRYING'}
			>
				Disconnect
			</button>
		</ConnectedScreenView>
	)
}
