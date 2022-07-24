/*
 =================================================
  APPLICATION STATE LOGIC
  WARNING: This should only ever be used by the Store.provider component
 =================================================
* */
import { BaseDirectory, writeTextFile } from '@tauri-apps/api/fs'
import { sendNotification } from '@tauri-apps/api/notification'
import { Reducer, useEffect, useReducer, useRef, useState } from 'react'
import { appStatus, ServerStatus, userSettingsPath } from '../../app.config'
import { useGetNotificationPermission } from '../../utils/useGetNotificationPermission'
import { UserSettings } from '../../utils/useSettings'
import { IconType } from '../UI/Icon/fa.defaults'
import { StatusHistory, Store } from './Store.provider'

type ReducerState = {
	statusMsg: string
	statusIcon: IconType
	history: StatusHistory[]
}
const reducer: Reducer<ReducerState, ServerStatus> = (state, serverStatus) => {
	const newState = { ...state }

	const { status, icon } = appStatus[serverStatus] || {}

	if (status) newState.statusMsg = status
	if (icon) newState.statusIcon = icon

	if (state.history[state.history.length - 1].status !== serverStatus)
		newState.history = [
			...state.history,
			{
				isoTimestamp: new Date().toISOString(),
				status: serverStatus,
			},
		]

	return newState
}

export const useAppState = (): Store => {
	const [{ statusMsg, statusIcon, history }, dispatch] = useReducer(reducer, {
		statusMsg: 'Ready',
		statusIcon: 'circle',
		history: [
			{
				isoTimestamp: new Date().toISOString(),
				status: 'READY',
			},
		],
	})

	const [status, setStatus] = useState<ServerStatus>('READY')
	const [systemErr, setSystemErr] = useState<string | null>(null)
	const [userSettings, setUserSettings] = useState<UserSettings | null>(null)
	const { granted } = useGetNotificationPermission()

	const writing = useRef(false)

	useEffect(() => {
		const writeSettingsFile = async () => {
			writing.current = true
			try {
				writeTextFile(
					{ path: userSettingsPath, contents: JSON.stringify(userSettings) },
					{ dir: BaseDirectory.Home }
				)

				if (!userSettings && granted)
					sendNotification({
						title: 'SUCCESS',
						body: 'Settings Saved!',
					})
				writing.current = false
			} catch (err: any) {
				setSystemErr(err)
				writing.current = false
			}
		}

		if (userSettings && status === 'CONNECTED' && !writing.current) writeSettingsFile()

		// eslint-disable-next-line react-hooks/exhaustive-deps
	}, [userSettings, status])

	useEffect(() => {
		dispatch(status)

		// Do some extra stuff depending on the status...
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
		}

		// eslint-disable-next-line react-hooks/exhaustive-deps
	}, [status])

	return {
		status,
		statusIcon,
		statusMsg,
		systemErr,
		userSettings,
		history,
		setStatus,
		setSystemErr,
		setUserSettings,
	}
}
