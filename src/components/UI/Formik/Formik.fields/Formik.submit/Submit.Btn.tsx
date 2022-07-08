import { HTMLProps } from 'react'
import styled, { css } from 'styled-components'
import { Spinner } from '../../../Spinner'

const submitBtnStyles = css`
	padding: 0.8em 1.5em;
	display: flex;
	align-items: center;
	justify-content: center;
	text-transform: uppercase;
	border: solid 2px;
	transition: all ${props => props.theme.times.tranS};
	background: ${p => p.theme.colors.white.val};

	&:hover {
		background: ${p => p.theme.colors.secondary.val};
		color: ${p => p.theme.colors.white.val};
	}
	&:focus {
		background: ${p => p.theme.colors.secondary.light().val};
		color: ${p => p.theme.colors.white.val};
	}

	&:disabled {
		color: ${p => p.theme.colors.disabled.val};
		border-color: ${p => p.theme.colors.disabled.val};
		&:hover,
		&:focus {
			color: ${p => p.theme.colors.disabled.val};
			border-color: ${p => p.theme.colors.disabled.val};
			background: ${p => p.theme.colors.white.val};
		}
	}
`

const SubmitBtnView = styled.button`
	${submitBtnStyles}
`

export type SubmitBtnProps = Pick<
	HTMLProps<HTMLButtonElement>,
	'onClick' | 'children' | 'type' | 'title' | 'disabled' | 'className'
> & {
	isSubmitting?: boolean
}
export const SubmitBtn = ({
	children,
	isSubmitting,
	disabled,
	className,
	...props
}: SubmitBtnProps) => {
	return (
		<SubmitBtnView
			className={`submit-btn${className ? ` ${className}` : ''}`}
			{...props}
			disabled={isSubmitting || disabled}
			type='submit'
		>
			{isSubmitting ? <Spinner height='sm' /> : null}
			{children || 'Save'}
		</SubmitBtnView>
	)
}
