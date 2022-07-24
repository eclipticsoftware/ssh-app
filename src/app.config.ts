import { IconType } from './components/UI/Icon/fa.defaults'

export const canPrint = true

export const userSettingsPath = 'eclo-ssh-client-user-settings.json'

export type AppStatus = {
	[x in ServerStatus]: {
		status: string
		icon: IconType
	}
}

export const appStatus = {
	/**
	 *  Disconnected state (either initial app state, or after disconnecting)
	 * */
	READY: {
		status: 'Ready',
		icon: 'circle',
	},

	/**
	 *  SSH connection proceedure running
	 * */
	CONNECTING: {
		status: 'Connecting',
		icon: 'circle',
	},

	/**
	 *  SSH connected
	 * */
	CONNECTED: {
		status: 'Connected',
		icon: 'ok',
	},

	/**
	 *  Attempting to reconnect to SSH after connection was dropped
	 * */
	RETRYING: {
		status: 'Reconnecting',
		icon: 'alert',
	},

	/**
	 *  Connection was unable to be reestablished after attempting reconnects
	 * */
	DROPPED: {
		status: 'Connection Dropped',
		icon: 'err',
	},

	/**
	 *  Server at IP address was unable to be reached
	 * */
	UNREACHABLE: {
		status: 'No Server Found',
		icon: 'err',
	},

	/**
	 *  Server at IP address denied the connection
	 * */
	DENIED: {
		status: 'Invalid Creds',
		icon: 'err',
	},

	/**
	 *  Bad configuration parameters were passed to server
	 * */
	BAD_CONFIG: {
		status: 'Invalid Params',
		icon: 'err',
	},

	/**
	 *  Server error
	 * */
	ERROR: {
		status: 'Server Error',
		icon: 'err',
	},

	/**
	 *  Unknown process exit error
	 * */
	UNKNOWN: {
		status: 'Unknown Error',
		icon: 'err',
	},
} as const

export type ServerStatus = keyof typeof appStatus

export const constants = {
	/**
	 *  Initial Connection Invocation
	 *  initiate ssh connection invokation name
	 * */
	startTunnel: 'start_tunnel',

	/**
	 *  Disconnect Invocation
	 * 	initiate ssh disconnection invokation name
	 * */
	endTunnel: 'end_tunnel',

	/**
	 *  Server Status Listener
	 * */
	tunnelStatus: 'tunnel_status',
}
