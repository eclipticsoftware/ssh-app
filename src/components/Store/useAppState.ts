import { sendNotification } from '@tauri-apps/api/notification'
import { Reducer, useEffect, useReducer, useState } from 'react'
import { ServerStatus } from '../../app.config'
import { useGetNotificationPermission } from '../../utils/useGetNotificationPermission'
import { IconType } from '../UI/Icon/fa.defaults'
import { Store } from './Store.provider'

type ReducerState = {
	statusMsg: string
	statusIcon: IconType
}
const reducer: Reducer<ReducerState, ServerStatus> = (state, serverStatus) => {
	const newState = { ...state }
	switch (serverStatus) {
		case ServerStatus.disconnected:
			newState.statusMsg = 'Ready'
			newState.statusIcon = 'circle'
			break
		case ServerStatus.connecting:
			newState.statusMsg = 'Connecting'
			newState.statusIcon = 'circle'
			break
		case ServerStatus.connected:
			newState.statusMsg = 'Connected'
			newState.statusIcon = 'ok'
			break
		case ServerStatus.retrying:
			newState.statusMsg = 'Reconnecting'
			newState.statusIcon = 'alert'
			break
		case ServerStatus.dropped:
			newState.statusMsg = 'Connection Dropped'
			newState.statusIcon = 'err'
			break
		case ServerStatus.badConfig:
			newState.statusMsg = 'Invalid Parameters'
			newState.statusIcon = 'err'
			break
		case ServerStatus.unreachable:
			newState.statusMsg = 'Server Unreachable'
			newState.statusIcon = 'err'
			break
		case ServerStatus.denied:
			newState.statusMsg = 'Invalid Credentials'
			newState.statusIcon = 'err'
			break

		default:
			break
	}

	return newState
}

export const useAppState = (): Store => {
	const [{ statusMsg, statusIcon }, dispatch] = useReducer(reducer, {
		statusMsg: 'Ready',
		statusIcon: 'circle',
	})

	const [status, setStatus] = useState<ServerStatus>('DISCONNECTED')
	const [systemErr, setSystemErr] = useState<string | null>(null)
	const { granted } = useGetNotificationPermission()

	useEffect(() => {
		dispatch(status)

		if (status === ServerStatus.connected) {
			if (granted)
				sendNotification({
					title: 'SUCCESS',
					body: 'SSH Connected!',
				})
		} else if (status === ServerStatus.dropped) {
			if (granted)
				sendNotification({
					title: 'ERROR',
					body: 'SSH Connection Dropped!',
				})
		} else if (status === ServerStatus.denied) {
			setSystemErr('Incorrect username or bad ssh key')

			if (granted)
				sendNotification({
					title: 'ERROR',
					body: 'Invalid Credentials!',
				})
		} else if (status === ServerStatus.unreachable) {
			setSystemErr('Incorrect IP Address')

			if (granted)
				sendNotification({
					title: 'ERROR',
					body: 'Server Unavailable',
				})
		} else if (status === ServerStatus.retrying) {
			if (granted)
				sendNotification({
					title: 'INTERRUPTION',
					body: 'SSH Connection Interrupted!',
				})
		} else if (status?.includes(ServerStatus.badConfig)) {
			const errMsg = status.substring(10)
			setSystemErr(`Invalid parameter(s): ${errMsg}`)
		}
	}, [status])

	return {
		status,
		statusIcon,
		statusMsg,
		systemErr,
		setStatus,
		setSystemErr,
	}
}
