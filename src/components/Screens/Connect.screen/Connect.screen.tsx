import { BaseDirectory, writeTextFile } from '@tauri-apps/api/fs'
import { sendNotification } from '@tauri-apps/api/notification'
import { invoke } from '@tauri-apps/api/tauri'
import styled, { css } from 'styled-components'
import * as Yup from 'yup'
import { constants, userSettingsPath } from '../../../app.config'
import { useGetNotificationPermission } from '../../../utils/useGetNotificationPermission'
import { useSettings } from '../../../utils/useSettings'
import useState from '../../../utils/useState'
import { ErrorBlock, ErrorBlockErr } from '../../UI/ErrorBlock'
import { FormikSelectFile } from '../../UI/Formik/Formik.fields/Formik.select.file'
import { FormikSubmitBtn } from '../../UI/Formik/Formik.fields/Formik.submit'
import { FormikText } from '../../UI/Formik/Formik.fields/Formik.text'
import { FormikForm } from '../../UI/Formik/Formik.form'
import { Icon } from '../../UI/Icon'
import { Spinner } from '../../UI/Spinner'

export const connectScreenStyles = css`
	.submit-btn {
		margin-top: 1em;
	}
`

const ConnectScreenView = styled.div`
	${connectScreenStyles}
`

const validationSchema = Yup.object().shape({
	host: Yup.string().required('Please enter an IP Address'),
	user: Yup.string().required('Please enter a username'),
	port: Yup.string().required('Please enter a port to forward the connection to'),
	keyPath: Yup.string().required('Please select an SSH Key file'),
})

export type ConnectScreenProps = {
	unknownErr: string | null
	connecting: boolean
}
export const ConnectScreen = ({ unknownErr, connecting }: ConnectScreenProps): JSX.Element => {
	const [settingsSaveErr, setSettingsErr] = useState<ErrorBlockErr | null>(null, 'settingsSaveErr')
	const [connectionErr, setConnectionErr] = useState<ErrorBlockErr | null>(null, 'connectionErr')

	const { loading, settings, error: settingsErr } = useSettings()
	const { granted } = useGetNotificationPermission()

	const initialVals = settings

	const onSubmit = async (vals: typeof initialVals) => {
		try {
			await writeTextFile(
				{ path: userSettingsPath, contents: JSON.stringify(vals) },
				{ dir: BaseDirectory.Home }
			)

			if (!settings && granted)
				sendNotification({
					title: 'SUCCESS',
					body: 'Settings Saved!',
				})
		} catch (err: any) {
			setSettingsErr(err?.message ?? err)
		}
		try {
			const { keyPath, ...data } = vals
			const res = await invoke(constants.startTunnel, {
				settings: {
					...data,
					key_path: keyPath,
				},
			})
			if (res === constants.unreachable) {
				setConnectionErr('Incorrect IP Address')
			} else if (res === constants.denied) {
				setConnectionErr('Incorrect username or bad ssh key')
			}
		} catch (err: any) {
			// console.log('Connection error: ', err)
			setConnectionErr(err)
		}
	}

	const error = settingsSaveErr || connectionErr

	return (
		<ConnectScreenView>
			{settingsErr ? <ErrorBlock error={settingsErr} /> : null}
			{loading ? (
				<Spinner />
			) : (
				<FormikForm
					initialValues={initialVals}
					onSubmit={onSubmit}
					validationSchema={validationSchema}
					enableReinitialize
				>
					{unknownErr ? <ErrorBlock error={unknownErr} /> : null}
					<FormikText name='host' config={{ label: 'IP Address (host)', isReq: true }} />
					<FormikText name='user' config={{ label: 'Username (user)', isReq: true }} />
					<FormikText name='port' config={{ label: 'Local Port (to forward to)', isReq: true }} />
					<FormikSelectFile name='keyPath' config={{ label: 'SSH Key', isReq: true }} />
					<hr />
					{error ? <ErrorBlock error={error} /> : null}
					{!loading && !settings?.host ? (
						<p className='no-settings-helper-text'>
							The next time you connect we will save your settings for future connections!
						</p>
					) : null}
					<FormikSubmitBtn isSubmitting={connecting}>
						<Icon padRight type='connect' /> Connect
					</FormikSubmitBtn>
				</FormikForm>
			)}
		</ConnectScreenView>
	)
}
