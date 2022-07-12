export const canPrint = true

export const userSettingsPath = 'eclo-ssh-client-user-settings.json'

export const constants = {
	// start ssh connection invokation name
	startTunnel: 'start_tunnel',

	// end ssh connection invokation name
	endTunnel: 'end_tunnel',

	// error listener name
	tunnelErr: 'tunnel_error',

	// success listener name
	tunnelSuccess: 'tunnel_connected',

	// retrying response
	retrying: 'RETRYING',

	// dropped response
	dropped: 'DROPPED',

	// error during initial connection
	// server exists but credentials were bad
	denied: 'DENIED',

	// error during initial connection
	// server could not be found (bad IP address)
	unreachable: 'UNREACHABLE',
}
