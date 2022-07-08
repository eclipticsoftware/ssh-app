import { css } from 'styled-components'

export const fieldInputStyles = css`
	border: none;
	outline: none;
	box-shadow: none;
	padding: 0.5em;
	font-size: 16px;
	color: ${props => props.theme.colors.text.val};
	background: none;
	width: 100%;
	text-align: left;
	border-radius: 0;

	&input[type='date'],
	&input[type='select'] {
		align-items: flex-start;
		display: block;
	}

	&:disabled {
		border-color: ${props => props.theme.colors.lightGrey.val} !important;
		color: ${props => props.theme.colors.lightGrey.val} !important;
	}
`

export const underlineFieldStyles = css`
	border-bottom: solid 2px ${props => props.theme.colors.grey.val};
`

export const boxFieldStyles = css`
	background: ${props => props.theme.colors.white.val};
	position: relative;

	input,
	select,
	textarea {
		border: solid 1px ${p => p.theme.colors.black.val};
		&:focus {
			border-color: ${p => p.theme.colors.primary.val};
		}

		&::placeholder {
			color: ${props => props.theme.colors.lightGrey.val};
		}
	}

	&:before {
		content: '';
		display: block;
		position: absolute;
		top: 0;
		left: 0;
		height: 100%;
		width: 5px;
		max-width: 0;
		background: ${p => p.theme.colors.secondary.val};
		transition: all ${props => props.theme.times.tranS};
		z-index: 1;
	}

	&:hover {
		&:before {
			max-width: 4px;
		}
	}
`

export const labelStyles = css`
	font-size: 0.8rem;
	line-height: 1rem;
	color: ${props => props.theme.colors.text.light().val};
`

export const fieldStyles = css`
	margin-bottom: 0.5em;

	.required-asterix {
		color: ${props => props.theme.colors.err.val};
		line-height: inherit;
		margin-left: 2px;
	}

	&.__space-bottom {
		margin-bottom: 2em;
	}
	&.__space-right {
		margin-right: 2em;
	}

	.label-content {
		display: inline-block;
		${labelStyles}
	}
	input,
	select,
	textarea {
		${fieldInputStyles}
	}

	&.__underline {
		.field-element {
			${underlineFieldStyles}
		}
	}

	&.__box {
		.field-element {
			${boxFieldStyles}
		}
	}

	&.label-pos-left {
		label {
			display: inline-flex;
			flex-direction: row;
			align-items: center;
			justify-content: flex-end;
		}
		input {
			width: auto;
			margin-right: 0.5em;
		}
	}

	&.label-pos-bottom {
		label {
			display: inline-flex;
			flex-direction: column-reverse;
		}
	}

	&.label-pos-right {
		label {
			display: inline-flex;
			align-items: flex-start;
			flex-direction: row-reverse;
			.label-content {
				display: inline-flex;
				align-items: flex-start;
				flex-direction: row-reverse;
				padding-left: 0.5em;
			}
			.required-asterix {
				margin: 0;
				margin-right: 2px;
			}
		}
	}

	&.__disabled {
		.field-element {
			&:before {
				display: none;
			}
		}
	}

	&.__suppress-styles {
		.field-element {
			background: none;

			&:before {
				display: none;
			}
		}
	}
`
