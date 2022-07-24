import { invoke } from '@tauri-apps/api'
import styled, { css } from 'styled-components'
import { constants } from '../../../app.config'
import { useStore } from '../../Store/Store.provider'
import { ErrorBlock } from '../../UI/ErrorBlock'

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

export type ConnectedScreenProps = {}

export const ConnectedScreen = (_: ConnectedScreenProps): JSX.Element => {
	const { status, systemErr } = useStore()

	const disconnectHandler = async () => {
		try {
			await invoke(constants.endTunnel)
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
