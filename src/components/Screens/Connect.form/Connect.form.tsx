import {BaseDirectory, writeTextFile} from '@tauri-apps/api/fs'
import {sendNotification} from '@tauri-apps/api/notification'
import {invoke} from '@tauri-apps/api/tauri'
import styled, {css} from 'styled-components'
import * as Yup from 'yup'
import {userSettingsPath} from '../../../app.config'
import {useGetNotificationPermission} from '../../../utils/useGetNotificationPermission'
import {useSettings} from '../../../utils/useSettings'
import useState from '../../../utils/useState'
import {ErrorBlock, ErrorBlockErr} from '../../UI/ErrorBlock'
import {FormikSelectFile} from '../../UI/Formik/Formik.fields/Formik.select.file'
import {FormikSubmitBtn} from '../../UI/Formik/Formik.fields/Formik.submit'
import {FormikText} from '../../UI/Formik/Formik.fields/Formik.text'
import {FormikForm} from '../../UI/Formik/Formik.form'
import Spinner from '../../UI/Spinner/Spinner'


  
export const connectFormStyles = css`
  

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
  onConnect: () => void
  onError: (error: ErrorBlockErr) => void
}
export const ConnectForm = ({onConnect, onError}: ConnectFormProps): JSX.Element => {
  const [settingsSaveErr, setSettingsErr] = useState<ErrorBlockErr | null>(null, 'settingsSaveErr')
  const [connectionErr, setConnectionErr] = useState<ErrorBlockErr | null>(null, 'connectionErr')

  const {loading, settings} = useSettings()
  const {granted} = useGetNotificationPermission()

  const initialVals = settings
  
  const onSubmit = async (vals: typeof initialVals) => {
    try {
      await writeTextFile({path: userSettingsPath, contents: JSON.stringify(vals)}, {dir: BaseDirectory.Home})

      if(granted) sendNotification({
        title: 'SUCCESS',
        body: 'Settings Saved!',
      })
    } catch (err: any) {
      setSettingsErr(err)
    }
    try {
      const {keyPath, ...data} = vals
      const res = await invoke('start_tunnel', {
        settings: {
          ...data,
          key_path: keyPath
        }
      })
      console.log('connection res: ', res)

      if(granted) sendNotification({
        title: 'SUCCESS',
        body: 'Connected!',
      })
      onConnect()
    } catch (err: any) {
      console.log('Connection error: ', err)
      setConnectionErr(err)
      onError(err)
    }
  }

  const error = settingsSaveErr || connectionErr

  return (
    <ConnectFormView>
      <div className="board">

      {loading ? <Spinner /> : (
        <FormikForm
        initialValues={initialVals}
        onSubmit={onSubmit}
        validationSchema={validationSchema}
        enableReinitialize
      >
        <FormikText name='host' config={{label: 'IP Address', isReq: true}} />
        <FormikText name='user' config={{label: 'Username', isReq: true}} />
        <FormikText name='port' config={{label: 'Local Port (to forward to)', isReq: true}} />
        <FormikSelectFile name='keyPath' config={{label: 'SSH Key', isReq: true}} />
        <hr />
        {error ? <ErrorBlock error={error} /> : null}
        <FormikSubmitBtn>Connect</FormikSubmitBtn>
      </FormikForm>
      )}
      </div>
    </ConnectFormView>
  )
}