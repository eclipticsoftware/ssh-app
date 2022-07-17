const fs = require('fs')
const path = require('path')
require('dotenv').config()

const { getOctokit, context } = require('@actions/github')

const config = JSON.parse(fs.readFileSync('./src-tauri/tauri.conf.json', 'utf8'))

const VERSION = config?.package?.version
const latestFilename = 'latest.json'

/**
 *  This was built based on a pull request that is currently waiting to be added to the tauri action project:
 *  Pull request: https://github.com/tauri-apps/tauri-action/pull/287/commits/0ebff2eb06ee1261374570118d59970522dac955
 *  NOTE: This may not be necessary once the PR has been accepted and merged.
 * */

async function main() {
	const TOKEN = process.env.GITHUB_TOKEN

	const releaseId = process.env.RELEASE_ID

	if (TOKEN === undefined) {
		throw new Error('GITHUB_TOKEN is required')
	}

	if (releaseId === undefined) {
		throw new Error('RELEASE_ID is required')
	}

	const github = getOctokit(TOKEN)

	const latestFilePath = path.resolve(process.cwd(), latestFilename)

	const latest = {
		version: `v${VERSION}`,
		notes: 'Alpha version',
		pub_date: new Date().toISOString(),
		platforms: {},
		// platforms: {
		// 	darwinx86_64: {
		// 		signature: '',
		// 		url: `https://github.com/eclipticsoftware/ssh-app/releases/download/${TAG}/ssh-app_${VERSION}_.tar.gz`,
		// 	},
		// 	darwinaarch64: {
		// 		signature: '',
		// 		url: `https://github.com/eclipticsoftware/ssh-app/releases/download/${TAG}/ssh-app_${VERSION}_.tar.gz`,
		// 	},
		// 	windowsx86_64: {
		// 		signature: '',
		// 		url: `https://github.com/eclipticsoftware/ssh-app/releases/download/v${VERSION}/ssh-app.x64.msi.zip`,
		// 	},
		// },
	}

	const assets = await github.rest.repos.listReleaseAssets({
		owner: context.repo.owner,
		repo: context.repo.repo,
		release_id: releaseId,
	})

	const macUrl = assets.data
		// .filter(e => assetNames.has(e.name))
		.find(s => s?.name?.endsWith('.tar.gz'))?.browser_download_url

	const windowsUrl = assets.data
		// .filter(e => assetNames.has(e.name))
		.find(s => s?.name?.endsWith('.zip'))?.browser_download_url

	if (macUrl) {
		const macSigFile = assets.data.find(s => s?.name?.endsWith('.gz.sig'))?.browser_download_url
		const macPlatform = {
			signature: macSigFile ? await fetch(macSigFile).toString() : undefined,
			url: macUrl,
		}

		latest.platforms[`darwinx86_64`] = macPlatform
		latest.platforms[`darwinaarch64`] = macPlatform
	}

	if (windowsUrl) {
		const winSigFile = assets.data.find(s => s?.name?.endsWith('.zip.sig'))?.browser_download_url
		const winPlatform = {
			signature: winSigFile ? await fetch(winSigFile).toString() : undefined,
			url: windowsUrl,
		}

		latest.platforms[`windowsx86_64`] = winPlatform
	}

	// const fileName = GH_WORKSPACE_PATH ? join(GH_WORKSPACE_PATH, latestFilename) : latestFilename

	console.log('macUrl: ', macUrl)
	console.log('windowsUrl: ', windowsUrl)
	console.log('latestFilePath: ', latestFilePath)

	fs.writeFileSync(latestFilePath, JSON.stringify(latest, null, 2))

	/**
	 *  Upload file
	 * */

	// ! We will store this in a gist not in the release

	// const contentLength = filePath => fs.statSync(filePath).size

	// const headers = {
	// 	'content-type': 'application/zip',
	// 	'content-length': contentLength(latestFilePath),
	// }

	// const ext = path.extname(latestFilePath)
	// const filename = path.basename(latestFilePath).replace(ext, '')
	// const assetName = path.dirname(latestFilePath).includes(`target${path.sep}debug`)
	// 	? `${filename}-debug${ext}`
	// 	: `${filename}${ext}`
	// console.log(`Uploading ${assetName}...`)
	// await github.rest.repos.uploadReleaseAsset({
	// 	headers,
	// 	name: assetName,
	// 	// https://github.com/tauri-apps/tauri-action/pull/45
	// 	// @ts-ignore error TS2322: Type 'Buffer' is not assignable to type 'string'.
	// 	data: fs.readFileSync(latestFilePath),
	// 	owner: context.repo.owner,
	// 	repo: context.repo.repo,
	// 	release_id: releaseId,
	// })
}

main()

// Mac zip download link:
// https://github.com/eclipticsoftware/ssh-app/releases/download/untagged-3c1507b2d46ce658c719/ssh-app.app.tar.gz

// Windows download link:
// https://github.com/eclipticsoftware/ssh-app/releases/download/untagged-3c1507b2d46ce658c719/ssh-app_0.0.1_x64_en-US.msi

// NOTE: can't rely on storing the latest.json in the release because the url will change with each release - SOLUTION: we should store it in a gist
