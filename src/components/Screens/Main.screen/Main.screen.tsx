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
	const showConnectionScreen = status === 'OK' || status === 'RETRYING'
	const [unknownErr, setErr] = useState<string | null>(null)

	const onDisconnect: ConnectedScreenProps['onDisconnect'] = () => {
		setStatus(null)
	}

	const { granted } = useGetNotificationPermission()

	useEffect(() => {
		let cleanupErrListener: UnlistenFn
		let cleanupSuccessListener: UnlistenFn

		listen(constants.tunnelErr, e => {
			const msg = e.payload
			if (msg === constants.retrying) setStatus('RETRYING')
			else {
				setStatus('DROPPED')
				if (msg !== constants.dropped) setErr(msg as string)
			}
		}).then(handler => (cleanupErrListener = handler))

		listen(constants.tunnelSuccess, () => {
			setStatus('OK')
			if (granted)
				sendNotification({
					title: 'SUCCESS',
					body: 'Connected!',
				})
		}).then(handler => (cleanupSuccessListener = handler))

		return () => {
			if (typeof cleanupErrListener === 'function') cleanupErrListener()
			if (typeof cleanupSuccessListener === 'function') cleanupSuccessListener()
		}
	}, [])

	return (
		<MainScreenView>
			<Board boardHeader={<BoardHeader status={status} />}>
				{showConnectionScreen ? (
					<ConnectedScreen status={status} onDisconnect={onDisconnect} />
				) : (
					<ConnectForm unknownErr={unknownErr} />
				)}
			</Board>
		</MainScreenView>
	)
}
