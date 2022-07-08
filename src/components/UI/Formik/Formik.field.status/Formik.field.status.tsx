import {useField} from 'formik'
import styled, {css} from 'styled-components'

export const formikFieldStatusStyles = css`
	font-size: 0.9em;
	.error {
		color: ${props => props.theme.colors.err.val};
	}
`

const FormikFieldStatusView = styled.div`
	${formikFieldStatusStyles}
`

export type FormikFieldStatusProps = {
	name: string
}
export const FormikFieldStatus = ({ name }: FormikFieldStatusProps) => {
	// eslint-disable-next-line @typescript-eslint/no-unused-vars
	const [_, { touched, error }] = useField({ name })
	return (
		<FormikFieldStatusView className='field-status'>
			{touched && error ? (
				<span className='error' color='err'>
					{error}
				</span>
			) : null}
		</FormikFieldStatusView>
	)
}
