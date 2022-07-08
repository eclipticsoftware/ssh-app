import {isPermissionGranted, requestPermission} from "@tauri-apps/api/notification"
import {useEffect, useState} from "react"


export const useGetNotificationPermission = () => {
  const [granted, setGranted] = useState(false)
  const [denied, setDenied] = useState(false)
  const [loading, setLoading] = useState(false)

  useEffect(() => {
    const ask = async () => {
      setLoading(true)

      const hasPermission = await isPermissionGranted()

      if(!hasPermission) {
        console.log('requesting permission')
        const permission = await requestPermission()
        if(permission === 'granted') setGranted(true)
        else setDenied(true)

      } else {
        console.log('has permission!')
        setGranted(true)
      }
      setLoading(false)
    }

    if(!loading && !granted && !denied) ask()
  },[granted, denied, loading])

  return {
    granted,
    denied,
    loading,
  }
}