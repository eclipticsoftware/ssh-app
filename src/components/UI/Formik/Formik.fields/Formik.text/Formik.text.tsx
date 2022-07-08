import { Field as FormikField, FieldConfig } from 'formik'
import styled, { css } from 'styled-components'
import { Field } from '../../../Field'
import { FormFieldProps } from '../../../Field/FormField.types'
import { FormikFieldStatus } from '../../Formik.field.status'

export const formikTextStyles = css``

const FormikTextView = styled.div`
	${formikTextStyles}
`

export type FormikTextProps = FieldConfig<string> & {
	config: FormFieldProps
	className?: string
	placeholder?: string
}
export const FormikText = ({ config: fieldProps, name, className, ...props }: FormikTextProps) => {
	return (
		<FormikTextView className={`formik-text${className ? ` ${className}` : ''}`}>
			<Field {...fieldProps} name={name} fieldStatus={<FormikFieldStatus name={name} />}>
				<FormikField {...props} name={name} />
			</Field>
		</FormikTextView>
	)
}
