import {Form, Formik, FormikConfig} from 'formik'
import styled, {css} from 'styled-components'


export const formikFormStyles = css`

`

const FormikFormView = styled.div`
	${formikFormStyles}
`

export type FormikFormProps<T> = Omit<FormikConfig<T>, 'initialValues'> & {
	initialValues: T
	className?: string
}
export function FormikForm<T>({ children, className, ...props }: FormikFormProps<T>) {
	return (
		<FormikFormView className={`formik-form${className ? ` ${className}` : ''}`}>
			<Formik {...props} validateOnMount>
				{typeof children === 'function' ? (
					props => <form onSubmit={props.handleSubmit}>{children(props)}</form>
				) : (
					<Form>{children}</Form>
				)}
			</Formik>
		</FormikFormView>
	)
}
