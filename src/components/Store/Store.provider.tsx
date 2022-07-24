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
	status: 'DISCONNECTED',
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
	const state = useAppState()

	useEffect(() => {
		let cleanupErrListener: UnlistenFn
		let cleanupSuccessListener: UnlistenFn

		listen(constants.tunnelStatus, e => {
			state.setStatus(e.payload as ServerStatus)
		}).then(handler => (cleanupSuccessListener = handler))

		return () => {
			if (typeof cleanupErrListener === 'function') cleanupErrListener()
			if (typeof cleanupSuccessListener === 'function') cleanupSuccessListener()
		}

		// eslint-disable-next-line react-hooks/exhaustive-deps
	}, [])

	return <Provider value={state}>{children}</Provider>
}
