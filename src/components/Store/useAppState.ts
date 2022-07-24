import { sendNotification } from '@tauri-apps/api/notification'
import { Reducer, useEffect, useReducer, useState } from 'react'
import { appStatus, ServerStatus } from '../../app.config'
import { useGetNotificationPermission } from '../../utils/useGetNotificationPermission'
import { IconType } from '../UI/Icon/fa.defaults'
import { Store } from './Store.provider'

type ReducerState = {
	statusMsg: string
	statusIcon: IconType
}
const reducer: Reducer<ReducerState, ServerStatus> = (state, serverStatus) => {
	const newState = { ...state }

	const { status, icon } = appStatus[serverStatus] || {}

	if (status) newState.statusMsg = status
	if (icon) newState.statusIcon = icon

	return newState
}

export const useAppState = (): Store => {
	const [{ statusMsg, statusIcon }, dispatch] = useReducer(reducer, {
		statusMsg: 'Ready',
		statusIcon: 'circle',
	})

	const [status, setStatus] = useState<ServerStatus>('READY')
	const [systemErr, setSystemErr] = useState<string | null>(null)
	const { granted } = useGetNotificationPermission()

	useEffect(() => {
		dispatch(status)

		if (status === 'CONNECTED') {
			if (granted)
				sendNotification({
					title: 'SUCCESS',
					body: 'SSH Connected!',
				})
		} else if (status === 'DROPPED') {
			if (granted)
				sendNotification({
					title: 'ERROR',
					body: 'SSH Connection Dropped!',
				})
		} else if (status === 'DENIED') {
			setSystemErr('Incorrect username or bad ssh key')

			if (granted)
				sendNotification({
					title: 'ERROR',
					body: 'Invalid Credentials!',
				})
		} else if (status === 'UNREACHABLE') {
			setSystemErr('Incorrect IP Address')

			if (granted)
				sendNotification({
					title: 'ERROR',
					body: 'Server Unavailable',
				})
		} else if (status === 'RETRYING') {
			if (granted)
				sendNotification({
					title: 'INTERRUPTION',
					body: 'SSH Connection Interrupted!',
				})
		} else if (status?.includes('BAD_CONFIG')) {
			const errMsg = status.substring(11)
			setSystemErr(`Invalid parameter(s): ${errMsg}`)
		} else if (status?.includes('ERROR')) {
			const errMsg = status.substring(6)
			setSystemErr(`System Error: ${errMsg}`)
		} else if (status?.includes('UNKNOWN')) {
			const errMsg = status.substring(8)
			setSystemErr(`Unknown Error: ${errMsg}`)
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
