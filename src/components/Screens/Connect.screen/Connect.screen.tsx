import { invoke } from '@tauri-apps/api/tauri'
import styled, { css } from 'styled-components'
import * as Yup from 'yup'
import { constants } from '../../../app.config'
import { useSettings } from '../../../utils/useSettings'
import { useStore } from '../../Store/Store.provider'
import { ErrorBlock } from '../../UI/ErrorBlock'
import { FormikSelectFile } from '../../UI/Formik/Formik.fields/Formik.select.file'
import { FormikSubmitBtn } from '../../UI/Formik/Formik.fields/Formik.submit'
import { submitBtnStyles } from '../../UI/Formik/Formik.fields/Formik.submit/Submit.Btn'
import { FormikText } from '../../UI/Formik/Formik.fields/Formik.text'
import { FormikForm } from '../../UI/Formik/Formik.form'
import { Icon } from '../../UI/Icon'
import { Spinner } from '../../UI/Spinner'

export const connectScreenStyles = css`
	.cancel-btn {
		border: none;
		outline: none;
		box-shadow: none;

		${submitBtnStyles}
	}
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

export type ConnectScreenProps = {}
export const ConnectScreen = (_: ConnectScreenProps): JSX.Element => {
	const { status, setSystemErr, setUserSettings } = useStore()
	const { loading, settings, error: settingsErr } = useSettings()

	const initialVals = settings

	const connecting = status === 'CONNECTING'

	const onSubmit = async (vals: typeof initialVals) => {
		setSystemErr(null)
		try {
			const { keyPath, ...data } = vals
			invoke(constants.startTunnel, {
				settings: {
					...data,
					key_path: keyPath,
				},
			})
			setUserSettings(vals)
		} catch (err: any) {
			// console.log('Connection error: ', err)
			setSystemErr(err)
		}
	}

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
					<FormikText name='host' config={{ label: 'IP Address (host)', isReq: true }} />
					<FormikText name='user' config={{ label: 'Username (user)', isReq: true }} />
					<FormikText name='port' config={{ label: 'Local Port (to forward to)', isReq: true }} />
					<FormikSelectFile name='keyPath' config={{ label: 'SSH Key', isReq: true }} />
					<hr />
					{!loading && !settings?.host ? (
						<p className='no-settings-helper-text'>
							The next time you connect we will save your settings for future connections!
						</p>
					) : null}
					{connecting ? (
						<button
							className='cancel-btn'
							onClick={e => {
								e.preventDefault()
								invoke(constants.endTunnel)
							}}
						>
							Cancel
						</button>
					) : (
						<FormikSubmitBtn isSubmitting={connecting}>
							<Icon padRight type='connect' /> Connect
						</FormikSubmitBtn>
					)}
				</FormikForm>
			)}
		</ConnectScreenView>
	)
}
