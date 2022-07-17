const fs = require('fs')
const path = require('path')
const { arch, platform } = require('os')
require('dotenv').config()

const { getOctokit, context } = require('@actions/github')

const config = JSON.parse(fs.readFileSync('./src-tauri/tauri.conf.json', 'utf8'))

const VERSION = config?.package?.version
const latestFilename = 'latest.json'

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

	const asset = assets.data.find(e => e.name === latestFilename)

	if (asset) {
		latest.platforms = (await (await fetch(asset.browser_download_url)).json()).platforms

		console.log(`${latest.platforms?.length} platforms found on existing asset`)

		// https://docs.github.com/en/rest/releases/assets#update-a-release-asset
		await github.rest.repos.deleteReleaseAsset({
			owner: context.repo.owner,
			repo: context.repo.repo,
			release_id: releaseId,
			asset_id: asset.id,
		})
	}

	const downloadUrl = assets.data
		// .filter(e => assetNames.has(e.name))
		.find(s => s?.name?.endsWith('.tar.gz') || s?.name?.endsWith('.zip'))?.browser_download_url

	if (downloadUrl) {
		// https://github.com/tauri-apps/tauri/blob/fd125f76d768099dc3d4b2d4114349ffc31ffac9/core/tauri/src/updater/core.rs#L856
		latest.platforms[
			`${platform().replace('win32', 'windows')}-${arch()
				.replace('arm64', 'aarch64')
				.replace('x64', 'x86_64')
				.replace('amd64', 'x86_64')
				.replace('arm', 'armv7')
				.replace('x32', 'i686')}`
		] = {
			// signature: sigFile ? readFileSync(sigFile).toString() : undefined, // TODO: implement this once we figure out how to get artifacts in here
			signature: '',
			url: downloadUrl,
		}
	}

	// const fileName = GH_WORKSPACE_PATH ? join(GH_WORKSPACE_PATH, latestFilename) : latestFilename

	console.log('downloadUrl: ', downloadUrl)
	console.log('latestFilePath: ', latestFilePath)

	fs.writeFileSync(latestFilePath, JSON.stringify(latest, null, 2))

	/**
	 *  Upload file
	 * */

	const contentLength = filePath => fs.statSync(filePath).size

	const headers = {
		'content-type': 'application/zip',
		'content-length': contentLength(latestFilePath),
	}

	const ext = path.extname(latestFilePath)
	const filename = path.basename(latestFilePath).replace(ext, '')
	const assetName = path.dirname(latestFilePath).includes(`target${path.sep}debug`)
		? `${filename}-debug${ext}`
		: `${filename}${ext}`
	console.log(`Uploading ${assetName}...`)
	await github.rest.repos.uploadReleaseAsset({
		headers,
		name: assetName,
		// https://github.com/tauri-apps/tauri-action/pull/45
		// @ts-ignore error TS2322: Type 'Buffer' is not assignable to type 'string'.
		data: fs.readFileSync(latestFilePath),
		owner: context.repo.owner,
		repo: context.repo.repo,
		release_id: releaseId,
	})
}

main()

// Mac zip download link:
// https://github.com/eclipticsoftware/ssh-app/releases/download/untagged-3c1507b2d46ce658c719/ssh-app.app.tar.gz

// Windows download link:
// https://github.com/eclipticsoftware/ssh-app/releases/download/untagged-3c1507b2d46ce658c719/ssh-app_0.0.1_x64_en-US.msi

// Latest.json
// https://github.com/eclipticsoftware/ssh-app/releases/download/untagged-86ac4099f7d9d32a510a/latest.json
