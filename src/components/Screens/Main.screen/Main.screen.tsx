import {useState} from 'react'
import styled, {css} from 'styled-components'
import {ConnectForm, ConnectFormProps} from '../Connect.form'
  
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
  
export type MainScreenProps = {
  
}
export const MainScreen = ({}: MainScreenProps): JSX.Element => {
  const [isConnected, setConnected] = useState(false)

  const onConnect: ConnectFormProps['onConnect'] = () => {

  }
  const onError:ConnectFormProps['onError'] = (err) => {

  }
  return (
    <MainScreenView>
      <ConnectForm onConnect={onConnect} onError={onError} />
    </MainScreenView>
  )
}