/*
 =================================================
  Wraps Font Awesome Icons
  maps FA icons to contextful names (e.g, "err" = faExclamationCircle)
  for mappings see: fonts/fa-library.js
  Docs: https://fontawesome.com/icons
 =================================================
* */
import {IconName, IconPrefix} from '@fortawesome/fontawesome-common-types'
import {FontAwesomeIcon} from '@fortawesome/react-fontawesome'
import isBoolean from 'lodash/isBoolean'
import styled from 'styled-components'
import {iconMap, IconType} from './fa.defaults'

type IconViewProps = {
	padRight?: string | boolean
	padLeft?: string | boolean
}

const IconView = styled.i<IconViewProps>`
	height: 1.2em;
	width: auto;
	display: inline-block;
	vertical-align: baseline;
	margin-left: ${props => (props.padLeft === true ? '0.4rem' : props.padLeft ? props.padLeft : 0)};
	margin-right: ${props =>
		props.padRight === true ? '0.4rem' : props.padRight ? props.padRight : 0};
	&& svg {
		width: inherit;
		height: inherit;
		color: inherit;
	}
`

type StylesOptions = {
	isFlipped?: boolean
	rotateRight?: boolean
	rotateLeft?: boolean
}

const styles = ({ isFlipped, rotateRight, rotateLeft }: StylesOptions) => {
	if (!isBoolean(isFlipped) && !isBoolean(rotateLeft) && !isBoolean(rotateRight)) return {}
	let transform = ''

	if (isBoolean(isFlipped)) transform = `scale(${isFlipped ? '-1' : '1'})`
	else if (rotateLeft) transform = `rotateZ(-90deg)`
	else if (rotateRight) transform = `rotateZ(90deg)`
	return { transform }
}

export type IconProps = IconViewProps &
	StylesOptions & {
		type: IconType
		className?: string
		title?: string
	}

export const Icon = ({
	type,
	padRight,
	padLeft,
	className,
	isFlipped,
	rotateRight,
	rotateLeft,
	title,
}:IconProps):JSX.Element => {
	const icon = iconMap[type || 'question']
	return (
		<IconView
			className={`icon ${type as string}${className ? ` ${className}` : ''}`}
			padLeft={padLeft}
			padRight={padRight}
			style={styles({ isFlipped, rotateRight, rotateLeft })}
			title={title}
		>
			<FontAwesomeIcon icon={icon as [IconPrefix, IconName]} />
		</IconView>
	)
}
