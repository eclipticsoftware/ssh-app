import styled, {css} from 'styled-components'
import * as Yup from 'yup'
import {FormikSelectFile} from '../../UI/Formik/Formik.fields/Formik.select.file'
import {FormikSubmitBtn} from '../../UI/Formik/Formik.fields/Formik.submit'
import {FormikText} from '../../UI/Formik/Formik.fields/Formik.text'
import {FormikForm} from '../../UI/Formik/Formik.form'


  
export const connectFormStyles = css`
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

  .submit-btn {
    margin-top: 1em;
  }
`
  
const ConnectFormView = styled.div`
  ${ connectFormStyles }
`
  
const validationSchema = Yup.object().shape({
  host: Yup.string().required('Please enter an IP Address'),
  user: Yup.string().required('Please enter a username'),
  port: Yup.string().required('Please enter a port to forward the connection to'),
  keyPath: Yup.string().required('Please select an SSH Key file'),
})

export type ConnectFormProps = {
  
}
export const ConnectForm = ({}: ConnectFormProps): JSX.Element => {
  const initialVals = {
    host: '',
    user: '',
    port: '',
    keyPath: ''
  }
  const onSubmit = async (vals: typeof initialVals) => {

  }
  return (
    <ConnectFormView>
      <div className="board">

      <FormikForm
        initialValues={initialVals}
        onSubmit={onSubmit}
        validationSchema={validationSchema}
      >
        <FormikText name='host' config={{label: 'IP Address', isReq: true}} />
        <FormikText name='user' config={{label: 'Username', isReq: true}} />
        <FormikText name='port' config={{label: 'Local Port (to forward to)', isReq: true}} />
        <FormikSelectFile name='keyPath' config={{label: 'SSH Key', isReq: true}} />
        <hr />
        <FormikSubmitBtn>Connect</FormikSubmitBtn>
      </FormikForm>
      </div>
    </ConnectFormView>
  )
}