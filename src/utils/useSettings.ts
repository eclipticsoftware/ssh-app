import { BaseDirectory, readTextFile } from '@tauri-apps/api/fs'
import { useEffect, useRef, useState } from 'react'
import { userSettingsPath } from '../app.config'

const defaultSettings = {
	host: '',
	user: '',
	port: '',
	keyPath: '',
}
type UserSettings = typeof defaultSettings

export const useSettings = () => {
	const [settings, setSettings] = useState<UserSettings>(defaultSettings)
	const [loading, setLoading] = useState(false)
	const [error, setError] = useState<string | null>(null)
	const isDone = useRef(false)

	useEffect(() => {
		const getFile = async () => {
			try {
				setLoading(true)

				const rawFile = await readTextFile(userSettingsPath, { dir: BaseDirectory.Home })

				if (rawFile) {
					const file = JSON.parse(rawFile)

					file &&
						setSettings(state => ({
							...state,
							...file,
						}))
					isDone.current = true
				}
				setLoading(false)
			} catch (err: any) {
				// We don't want to report any errors from this
				isDone.current = true
				setLoading(false)
				setError('Unable to find any saved user settings')
			}
		}
		if (!loading && !isDone.current) getFile()
	}, [loading, settings])

	return { settings, loading, error }
}
