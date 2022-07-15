const fs = require('fs')
require('dotenv').config()

const config = JSON.parse(fs.readFileSync('./src-tauri/tauri.conf.json', 'utf8'))

const VERSION = config?.package?.version
const TAG = '' // TODO: we need to get the tag somehow...

function main() {
	const updater = {
		version: `v${VERSION}`,
		notes: 'Alpha version',
		pub_date: new Date().toISOString(),
		platforms: {
			darwinx86_64: {
				signature: '',
				url: `https://github.com/eclipticsoftware/ssh-app/releases/download/${TAG}/ssh-app_${VERSION}_.tar.gz`,
			},
			darwinaarch64: {
				signature: '',
				url: `https://github.com/eclipticsoftware/ssh-app/releases/download/${TAG}/ssh-app_${VERSION}_.tar.gz`,
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

// https://github.com/eclipticsoftware/ssh-app/releases/download/untagged-3c1507b2d46ce658c719/ssh-app.app.tar.gz
