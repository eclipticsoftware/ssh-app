export const canPrint = true

export const userSettingsPath = 'eclo-ssh-client-user-settings.json'

export const ServerStatus = {
	/**
	 *  Initial app state
	 * */
	disconnected: 'DISCONNECTED',

	/**
	 *  SSH connection proceedure running
	 * */
	connecting: 'CONNECTING',

	/**
	 *  SSH connected
	 * */
	connected: 'CONNECTED',

	/**
	 *  SSH disconnection proceedure has run successfully
	 *  App state after disconnecting
	 * */
	exit: 'EXIT',

	/**
	 *  Attempting to reconnect to SSH after connection was dropped
	 * */
	retrying: 'RETRYING',

	/**
	 *  Connection was unable to be reestablished after attempting reconnects
	 * */
	dropped: 'DROPPED',

	/**
	 *  Bad configuration parameters were passed to server
	 * */
	badConfig: 'BAD_CONFIG',

	/**
	 *  Server at IP address was unable to be reached
	 * */
	unreachable: 'UNREACHABLE',

	/**
	 *  Server at IP address denied the connection
	 * */
	denied: 'DENIED',
} as const

type ServerStatusKeys = keyof typeof ServerStatus

export type ServerStatus = typeof ServerStatus[ServerStatusKeys]

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
