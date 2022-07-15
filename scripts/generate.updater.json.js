const fs = require('fs')
const { join } = require('path')
require('dotenv').config()

const config = JSON.parse(fs.readFileSync('./src-tauri/tauri.conf.json', 'utf8'))

const VERSION = config?.package?.version
const TAG = 'untagged-3c1507b2d46ce658c719' // TODO: we need to get the tag somehow...
const GH_WORKSPACE_PATH = process.env.GITHUB_WORKSPACE
const releaseId = process.env.RELEASE_ID
const releaseUrl = process.env.RELEASE_URL
const FILE_PATH = 'updater.json'

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

	const fileName = GH_WORKSPACE_PATH ? join(GH_WORKSPACE_PATH, FILE_PATH) : FILE_PATH

	console.log('fileName: ', fileName)
	console.log('releaseId: ', releaseId)
	console.log('releaseUrl: ', releaseUrl)

	fs.writeFileSync(fileName, JSON.stringify(updater, null, 2))
}

main()

// https://github.com/eclipticsoftware/ssh-app/releases/download/untagged-3c1507b2d46ce658c719/ssh-app.app.tar.gz
