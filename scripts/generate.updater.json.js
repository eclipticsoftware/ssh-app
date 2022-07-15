const fs = require('fs')
require('dotenv').config()

const VERSION = process.env.APP_VERSION

function main() {
	const updater = {
		version: `v${VERSION}`,
		notes: 'Alpha version',
		pub_date: new Date().toISOString(),
		platforms: {
			darwinx86_64: {
				signature: '',
				url: `https://github.com/eclipticsoftware/ssh-app/releases/download/v${VERSION}/ssh-app.tar.gz`,
			},
			darwinaarch64: {
				signature: '',
				url: `https://github.com/eclipticsoftware/ssh-app/releases/download/v${VERSION}/silicon/ssh-app.tar.gz`,
			},
			linuxx86_64: {
				signature: '',
				url: `https://github.com/eclipticsoftware/ssh-app/releases/download/v${VERSION}/ssh-appImage.tar.gz`,
			},
			windowsx86_64: {
				signature: '',
				url: `https://github.com/eclipticsoftware/ssh-app/releases/download/v${VERSION}/ssh-app.x64.msi.zip`,
			},
		},
	}

	fs.writeFileSync('./updater.json', JSON.stringify(updater, null, 2))
}

main()
