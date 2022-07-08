/*
 =================================================
	Form field wrapper component
	This component adds default/cohesive field styles and layout for any input that it wraps
 =================================================
* */
import { ReactNode } from 'react'
import styled from 'styled-components'
import { fieldStyles } from './Field.styles'

const FieldView = styled.div`
	${fieldStyles}
`

export type FieldProps = {
	label?: ReactNode
	labelPos?: 'top' | 'left' | 'bottom' | 'right'
	name?: string
	className?: string
	spaceBottom?: boolean
	spaceRight?: boolean
	displayType?: 'underline' | 'box'
	isReq?: boolean
	children: ReactNode
	fieldStatus?: ReactNode
	disabled?: boolean
	suppressStyles?: boolean
}
export const Field = ({
	children,
	className,
	label,
	labelPos = 'top',
	spaceBottom,
	spaceRight,
	displayType = 'box',
	isReq,
	fieldStatus,
	name,
	disabled,
	suppressStyles,
}: FieldProps) => {
	return (
		<FieldView
			className={`field${className ? ` ${className}` : ''}${spaceBottom ? ' __space-bottom' : ''}${
				spaceRight ? ' __space-right' : ''
			}${
				displayType === 'underline' ? ' __underline' : displayType === 'box' ? ' __box' : ''
			} label-pos-${labelPos} ${disabled ? ' __disabled' : ''}${
				suppressStyles ? ' __suppress-styles' : ''
			}`}
		>
			{label || isReq ? (
				<label htmlFor={name}>
					<div className='label-content'>
						{label}
						{isReq ? <span className='required-asterix'>*</span> : null}
					</div>
					<div className='field-element'>{children}</div>
				</label>
			) : (
				<div className='field-element'>{children}</div>
			)}
			{fieldStatus ? <div className='field-status'>{fieldStatus}</div> : null}
		</FieldView>
	)
}
