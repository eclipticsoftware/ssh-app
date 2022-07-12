export const canPrint = true

export const userSettingsPath = 'eclo-ssh-client-user-settings.json'

export const constants = {
	/**
	 *  Initial Connection Invocation
	 * */
	// start ssh connection invokation name
	startTunnel: 'start_tunnel',

	// connection invocation response
	// server could not be found (bad IP address)
	unreachable: 'UNREACHABLE',

	// connection invocation response
	// successfully connected
	success: 'SUCCESS',

	/**
	 *  Disconnect Invocation
	 * */
	// end ssh connection invokation name
	endTunnel: 'end_tunnel',

	/**
	 *  Connection Status Listener
	 * */
	// status listener name
	tunnelStatus: 'tunnel_status',

	// status listener response
	connected: 'CONNECTED',

	// status listener response
	retrying: 'RETRYING',

	// status listener response
	dropped: 'DROPPED',

	// connection invocation response
	// server exists but credentials were bad
	denied: 'DENIED',
}
