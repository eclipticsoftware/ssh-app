import { listen, UnlistenFn } from '@tauri-apps/api/event'
import { sendNotification } from '@tauri-apps/api/notification'
import { useEffect, useState } from 'react'
import styled, { css } from 'styled-components'
import { constants } from '../../../app.config'
import { useGetNotificationPermission } from '../../../utils/useGetNotificationPermission'
import { Board } from '../../UI/Board'
import { BoardHeader } from '../../UI/Board.header/Board.header'
import { ConnectScreen } from '../Connect.screen'
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

export type ConnectionStatus = 'OK' | 'DROPPED' | 'RETRYING' | 'ERROR' | 'DISCONNECTED'

export const MainScreen = (): JSX.Element => {
	const [status, setStatus] = useState<ConnectionStatus>('DISCONNECTED')
	const showConnectedScreen = status === 'OK' || status === 'RETRYING'
	const [unknownErr, setErr] = useState<string | null>(null)

	const onDisconnect: ConnectedScreenProps['onDisconnect'] = () => {
		setStatus('DISCONNECTED')
	}

	const { granted } = useGetNotificationPermission()

	useEffect(() => {
		let cleanupErrListener: UnlistenFn
		let cleanupSuccessListener: UnlistenFn

		listen(constants.tunnelStatus, e => {
			const payload = e.payload as string

			if (payload === constants.connected) {
				/**
				 *  SUCCESSFULLY CONNECTED
				 * */

				setStatus('OK')
				if (granted)
					sendNotification({
						title: 'SUCCESS',
						body: 'SSH Connected!',
					})
			} else if (payload === constants.dropped) {
				/**
				 *  CONNECTION DROPPED
				 * */

				setStatus('DROPPED')
				if (granted)
					sendNotification({
						title: 'ERROR',
						body: 'SSH Connection Dropped!',
					})
			} else if (payload === constants.retrying) {
				/**
				 *  ATTEMPTING TO RE-ESTABLISH CONNECTION
				 * */

				setStatus('RETRYING')
				if (granted)
					sendNotification({
						title: 'INTERRUPTION',
						body: 'SSH Connection Interrupted!',
					})
			} else if (payload === constants.disconnected) {
				/**
				 *  CONNECTION NOT YET ESTABLISHED
				 *  NOTE: This is both the state when the app opens
				 *  as well as when the connection is manually terminated by the user
				 * */

				setStatus('DISCONNECTED')
			} else if (payload?.includes(constants.badConfig)) {
				/**
				 *  ERROR DUE TO USER ENTERING INVALID CONFIG OPTIONS
				 * */

				const errMsg = payload.substring(10)
				setStatus('ERROR')
				setErr(`Invalid parameter(s): ${errMsg}`)
			}
		}).then(handler => (cleanupSuccessListener = handler))

		return () => {
			if (typeof cleanupErrListener === 'function') cleanupErrListener()
			if (typeof cleanupSuccessListener === 'function') cleanupSuccessListener()
		}

		// eslint-disable-next-line react-hooks/exhaustive-deps
	}, [])

	return (
		<MainScreenView>
			<Board boardHeader={<BoardHeader status={status} />}>
				{showConnectedScreen ? (
					<ConnectedScreen status={status} onDisconnect={onDisconnect} />
				) : (
					<ConnectScreen unknownErr={unknownErr} />
				)}
			</Board>
		</MainScreenView>
	)
}
