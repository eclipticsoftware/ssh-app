import { invoke } from '@tauri-apps/api'
import styled, { css } from 'styled-components'
import { constants } from '../../../app.config'
import { Board } from '../../UI/Board'
import { Icon } from '../../UI/Icon'
import { ConnectionStatus } from '../Main.screen'

export const connectedScreenStyles = css`
	.status {
		&.__connected {
			color: ${props => props.theme.colors.ok.val};
		}
		&.__reconnecting {
			color: ${props => props.theme.colors.grey.val};
		}
	}
`

const ConnectedScreenView = styled.div`
	${connectedScreenStyles}
`

export type ConnectedScreenProps = {
	status: ConnectionStatus
	onDisconnect: () => void
}

export const ConnectedScreen = ({ status, onDisconnect }: ConnectedScreenProps): JSX.Element => {
	const disconnectHandler = async () => {
		try {
			await invoke(constants.endTunnel)
			onDisconnect()
		} catch {}
	}
	return (
		<ConnectedScreenView>
			<Board boardTitle='NNR SSH CLIENT'>
				<div className='connection-info'>
					<h5>Status:</h5>
					<div className={`status __${status.toLowerCase()}`}>
						<Icon type={status === 'Connected' ? 'ok' : 'err'} />
						<span>{status === 'Connected' ? status : `${status}...`}</span>
					</div>
				</div>
				<button onClick={disconnectHandler} disabled={status === 'Connected'}>
					Disconnect
				</button>
			</Board>
		</ConnectedScreenView>
	)
}
