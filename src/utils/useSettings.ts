import {BaseDirectory, readTextFile} from "@tauri-apps/api/fs"
import {useEffect, useRef, useState} from "react"
import {userSettingsPath} from "../app.config"


const defaultSettings = {
  host: '',
  user: '',
  port: '',
  keyPath: ''
}
type UserSettings = typeof defaultSettings

export const useSettings = () => {
  const [settings, setSettings] = useState<UserSettings>(defaultSettings)
  const [loading, setLoading] = useState(false)
  const settingsLoaded = useRef(false)

  useEffect(() => {
    const getFile = async () => {
      try {
        setLoading(true)

        const rawFile = await readTextFile(userSettingsPath, {dir: BaseDirectory.Home})
        
        console.log('rawFile: ', rawFile)
        if(rawFile) {
          const file = JSON.parse(rawFile)
          console.log('file: ', file)

          file && setSettings((state) => ({
            ...state,
            ...file
          }))
          settingsLoaded.current = true
        }
        setLoading(false)
      } catch (err) {
        // We don't want to report any errors from this
        setLoading(false)
      }
      
    }
    if(!loading && !settingsLoaded.current) getFile()
  },[loading, settings])

  return {settings, loading}
}