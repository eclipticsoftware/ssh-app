import {listen} from '@tauri-apps/api/event'
import {useEffect, useState} from 'react'
import styled, {css} from 'styled-components'
import {ConnectForm, ConnectFormProps} from '../Connect.form'
import {ConnectedScreen, ConnectedScreenProps} from '../Connected.screen'
  
export const mainScreenStyles = css`
  width: 100%;
  height: 100%;
  min-width: 100vw;
  min-height: 100vh;;
  display: flex;
  align-items: center;
  justify-content: center;
  background: ${props => props.theme.colors.lightGrey.val};
  
  .board {
    background: ${props => props.theme.colors.white.val};
    border-radius: 10px;;
    max-width: 600px;
    padding: 2em 4em;
    box-shadow: 0 2px 3px ${props => props.theme.colors.grey.opacity(70).val};
  }
  hr {
    margin: 1em 0;
  }
`
  
const MainScreenView = styled.div`
  ${ mainScreenStyles }
`
export type ConnectionStatus = 'Connected' | 'Dropped' | 'Reconnecting...'

export const MainScreen = (): JSX.Element => {
  const [status, setStatus] = useState<ConnectionStatus | null>(null)
  const showConnectionScreen = !!status && status !== 'Dropped'

  const onConnect: ConnectFormProps['onConnect'] = () => {
    setStatus('Connected')
  }
  const onError:ConnectFormProps['onError'] = () => {
    setStatus('Dropped')
  }

  const onDisconnect: ConnectedScreenProps['onDisconnect'] = () => {
    setStatus(null)
  }

  useEffect(() => {
    listen('tunnel_error', (error) => {
      setStatus('Dropped')
    })
  },[])

  return (
    <MainScreenView>
      {showConnectionScreen ? <ConnectedScreen status={status} onDisconnect={onDisconnect} /> : (

      <ConnectForm onConnect={onConnect} onError={onError} status={status} />
      )}
    </MainScreenView>
  )
}