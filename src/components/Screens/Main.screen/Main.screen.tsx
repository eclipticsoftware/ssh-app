import { listen, UnlistenFn } from '@tauri-apps/api/event'
import { sendNotification } from '@tauri-apps/api/notification'
import { useEffect, useState } from 'react'
import styled, { css } from 'styled-components'
import { constants } from '../../../app.config'
import { useGetNotificationPermission } from '../../../utils/useGetNotificationPermission'
import { Board } from '../../UI/Board'
import { BoardHeader } from '../../UI/Board.header/Board.header'
import { ConnectForm } from '../Connect.form'
import { ConnectedScreen, ConnectedScreenProps } from '../Connected.screen'

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

export type ConnectionStatus = 'OK' | 'DROPPED' | 'RETRYING' | 'ERROR'

export const MainScreen = (): JSX.Element => {
	const [status, setStatus] = useState<ConnectionStatus | null>(null)
	const showConnectedScreen = status === 'OK' || status === 'RETRYING'
	const [unknownErr, setErr] = useState<string | null>(null)

	const onDisconnect: ConnectedScreenProps['onDisconnect'] = () => {
		setStatus(null)
	}

	const { granted } = useGetNotificationPermission()

	useEffect(() => {
		let cleanupErrListener: UnlistenFn
		let cleanupSuccessListener: UnlistenFn

		listen(constants.tunnelStatus, e => {
			if (e.payload === constants.connected) {
				setStatus('OK')
				if (granted)
					sendNotification({
						title: 'SUCCESS',
						body: 'Connected!',
					})
			} else if (e.payload === constants.dropped) {
				setStatus('DROPPED')
				if (granted)
					sendNotification({
						title: 'ERROR',
						body: 'Connection Dropped!',
					})
			} else if (e.payload === constants.retrying) {
				setStatus('RETRYING')
				if (granted)
					sendNotification({
						title: 'INTERRUPTION',
						body: 'Connection Interrupted!',
					})
			}
		}).then(handler => (cleanupSuccessListener = handler))

		return () => {
			if (typeof cleanupErrListener === 'function') cleanupErrListener()
			if (typeof cleanupSuccessListener === 'function') cleanupSuccessListener()
		}
	}, [])

	return (
		<MainScreenView>
			<Board boardHeader={<BoardHeader status={status} />}>
				{showConnectedScreen ? (
					<ConnectedScreen status={status} onDisconnect={onDisconnect} />
				) : (
					<ConnectForm unknownErr={unknownErr} />
				)}
			</Board>
		</MainScreenView>
	)
}
