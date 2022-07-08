import styled, {css} from 'styled-components'
  
export const connectedScreenStyles = css`
  
`
  
const ConnectedScreenView = styled.div`
  ${ connectedScreenStyles }
`
  
export type ConnectedScreenProps = {
  
}
export const ConnectedScreen = ({}: ConnectedScreenProps): JSX.Element => {
  const disconnectHandler = async () => {
    try {
      
    } catch (err) {
      
    }
  }
  return (
    <ConnectedScreenView>
      <div className="board">
        <h3>Connected!</h3>
        <button onClick={disconnectHandler}>Disconnect</button>
      </div>
    </ConnectedScreenView>
  )
}