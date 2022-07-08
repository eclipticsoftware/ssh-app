import styled, {css} from 'styled-components'
import {ConnectForm, ConnectFormProps} from '../Connect.form'
  
export const mainScreenStyles = css`
  
`
  
const MainScreenView = styled.div`
  ${ mainScreenStyles }
`
  
export type MainScreenProps = {
  
}
export const MainScreen = ({}: MainScreenProps): JSX.Element => {
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