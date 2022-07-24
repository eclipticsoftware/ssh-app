import { invoke } from '@tauri-apps/api'
import styled, { css } from 'styled-components'
import { constants } from '../../../app.config'
import { useStore } from '../../Store/Store.provider'

export const connectedScreenStyles = css`
	.info {
		padding: 1em;
		margin-bottom: 1em;
		color: ${props => props.theme.colors.grey.val};
		border: solid 1px ${props => props.theme.colors.grey.light(1).val};
		border-radius: 5px;

		.port {
			color: ${props => props.theme.colors.grey.dark(1).val};
		}
	}

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

export type ConnectedScreenProps = {}

export const ConnectedScreen = (_: ConnectedScreenProps): JSX.Element => {
	const { status, userSettings } = useStore()
	const { port } = userSettings || {}

	const disconnectHandler = async () => {
		try {
			await invoke(constants.endTunnel)
		} catch {}
	}

	return (
		<ConnectedScreenView>
			<div className='info'>
				{port ? (
					<div className='port-info'>
						Listening on localhost PORT: <span className='port'>{port}</span>
					</div>
				) : null}
			</div>
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
