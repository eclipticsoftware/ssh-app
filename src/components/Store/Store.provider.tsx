/*
 =================================================
  MANAGES AND PROVIDES APPLICATION STATE
  NOTE: Most app state is determined by signals emitted from the server
  However occasionally we need to update state from client side operations as well.
 =================================================
* */
import { listen, UnlistenFn } from '@tauri-apps/api/event'
import { createContext, Dispatch, ReactNode, useContext, useEffect } from 'react'
import { constants, ServerStatus } from '../../app.config'
import { IconType } from '../UI/Icon/fa.defaults'
import { useAppState } from './useAppState'

export type Store = {
	status: ServerStatus
	setStatus: Dispatch<ServerStatus>
	statusMsg: string
	systemErr: string | null
	setSystemErr: Dispatch<string | null>
	statusIcon: IconType
}

const initialStore: Store = {
	status: 'READY',
	setStatus: () => {},
	statusMsg: 'Ready',
	systemErr: null,
	setSystemErr: () => {},
	statusIcon: 'circle',
}

export const context = createContext(initialStore)

export const useStore = () => useContext(context)

const Provider = context.Provider

export type StoreProviderProps = {
	children: ReactNode
}
export const StoreProvider = ({ children }: StoreProviderProps): JSX.Element => {
	// We abstract the app state to a hook to keep this component cleaner
	const state = useAppState()

	useEffect(() => {
		let cleanupSuccessListener: UnlistenFn

		/**
		 *  Server Status Listener
		 * */
		listen(constants.tunnelStatus, e => {
			const signal = e.payload as string

			let serverStatus = signal

			// This means that the signal contains an error message
			// e.g.: "ERROR: something went wrong"
			if (signal.includes(':')) {
				// Parse the error message from the signal
				const err = signal.substring(signal.indexOf(':') + 2)
				state.setSystemErr(err)
				// Use the bit before the error message for the server status
				serverStatus = signal.substring(0, signal.indexOf(':'))
			}

			state.setStatus(serverStatus as ServerStatus)

			// Assign the unregister listener function for clean up purposes
		}).then(handler => (cleanupSuccessListener = handler))

		return () => {
			if (typeof cleanupSuccessListener === 'function') cleanupSuccessListener()
		}

		// eslint-disable-next-line react-hooks/exhaustive-deps
	}, [])

	return <Provider value={state}>{children}</Provider>
}
