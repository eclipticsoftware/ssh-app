import ClipLoader from 'react-spinners/ClipLoader'
import PulseLoader from 'react-spinners/PulseLoader'
import styled, {css} from 'styled-components'

const override = css``

const SpinnerWrap = styled.div<{ noBg?: boolean; invert?: boolean }>`
	position: absolute;
	top: 0;
	left: 0;
	right: 0;
	bottom: 0;
	display: flex;
	align-items: center;
	justify-content: center;
	z-index: 100;
	background: ${props =>
		props.noBg
			? 'none'
			: props.invert
			? props.theme.colors.grey.val
			: props.theme.colors.white.val};
`

const Spinner = ({
	type = 'dots',
	invert = false,
	color = null,
	noBg = false,
	className = null,
	...props
}) => {


	const spinnerProps = {
		color: '#808782',
		css: override,
		...props,
	} as any
	return (
		<SpinnerWrap
			className={`spinner-wrap${className ? ` ${className}` : ''}`}
			invert={invert}
			noBg={noBg}
		>
			{type === 'dots' ? (
				<PulseLoader {...spinnerProps} />
			) : type === 'circle' ? (
				<ClipLoader {...spinnerProps} />
			) : (
				<p>Loading...</p>
			)}
		</SpinnerWrap>
	)
}
export default Spinner
