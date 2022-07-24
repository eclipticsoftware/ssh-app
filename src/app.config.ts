export const canPrint = true

export const userSettingsPath = 'eclo-ssh-client-user-settings.json'

export const ServerStatus = {
	connecting: 'CONNECTING',

	// status listener response
	connected: 'CONNECTED',

	// status listener response
	retrying: 'RETRYING',

	// status listener response
	dropped: 'DROPPED',

	// status listener response
	disconnected: 'DISCONNECTED',

	// status listener response
	badConfig: 'BAD_CONFIG',

	// status listener response
	unreachable: 'UNREACHABLE',

	// status listener response
	// server exists but credentials were bad
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
