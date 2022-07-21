import { useFormikContext } from 'formik'
import { SubmitBtn, SubmitBtnProps } from './Submit.Btn'

export type FormikSubmitBtnProps = SubmitBtnProps & {
	className?: string
}

export const FormikSubmitBtn = ({
	className,
	isSubmitting: submitting,
	...props
}: FormikSubmitBtnProps) => {
	const { isSubmitting, isValid } = useFormikContext()

	return (
		<SubmitBtn
			className={`formik-submit${className ? ` ${className}` : ''}`}
			{...props}
			disabled={!isValid}
			isSubmitting={submitting || isSubmitting}
		/>
	)
}
