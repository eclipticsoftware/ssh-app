const fs = require('fs')
require('dotenv').config()

const config = JSON.parse(fs.readFileSync('./src-tauri/tauri.conf.json', 'utf8'))

const VERSION = config?.package?.version

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
