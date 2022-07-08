import ClipLoader from 'react-spinners/ClipLoader'
import PulseLoader from 'react-spinners/PulseLoader'
import styled, { useTheme } from 'styled-components'
import { spinnerStyles, SpinnerStylesProps } from './Spinner.styles'

type Sizes = 'sm' | 'med' | 'lg'
const SpinnerView = styled.div<SpinnerStylesProps>`
	${spinnerStyles}
`
const calcSize = (height: Sizes, type: Type) => {
	if (type === 'dots')
		return height === 'sm'
			? 8
			: height === 'med'
			? 12
			: height === 'lg'
			? 15
			: height === 'full'
			? 15
			: undefined
	else
		return height === 'sm'
			? 15
			: height === 'med'
			? 25
			: height === 'lg'
			? 35
			: height === 'full'
			? 15
			: undefined
}

type Type = 'dots' | 'circle'
export type SpinnerProps = {
	type?: Type
	noBg?: boolean
	isOverlay?: boolean
	height?: Sizes | 'full' | 'half' | '100%' | '30vh'
	invert?: boolean
	color?: string
	show?: boolean
	className?: string
	bgColor?: string
}
export const Spinner = ({
	type = 'dots',
	noBg,
	invert,
	isOverlay = true,
	height = 'med',
	color,
	show = true,
	className,
	bgColor,
}: SpinnerProps): JSX.Element => {
	const { color: colorFunc, colors } = useTheme()

	const spinnerProps = {
		color: color ? colorFunc(color).val : invert ? colors.offWhite.val : colors.grey.val,
		loading: show,
		size: calcSize(height as Sizes, type),
	}

	const { sizes } = useTheme()

	const headerHeight = sizes.header?.mobile.num || 80
	const footerHeight = sizes.footer?.mobile.num || 100
	const offset = headerHeight + footerHeight
	const unit = sizes.footer?.mobile.unit || 'px'

	const spinnerHeight =
		height === 'sm'
			? '60px'
			: height === 'med'
			? '100px'
			: height === 'lg'
			? '200px'
			: height === 'full'
			? `calc(100vh - ${offset}${unit})`
			: height === 'half'
			? '50vh'
			: height

	return (
		<SpinnerView
			height={spinnerHeight}
			className={`spinner${className ? ` ${className}` : ''}${invert && !color ? ' __invert' : ''}${
				noBg ? ' __no-bg' : ''
			}${isOverlay ? ' __overlay' : ''}`}
			style={{ background: noBg ? 'none' : bgColor || colors.white.val }}
		>
			{type === 'dots' ? (
				<PulseLoader {...spinnerProps} />
			) : type === 'circle' ? (
				<ClipLoader {...spinnerProps} />
			) : (
				`No spinner of type '${type}' found...`
			)}
		</SpinnerView>
	)
}
