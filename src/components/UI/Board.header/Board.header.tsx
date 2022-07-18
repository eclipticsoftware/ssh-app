import styled, { css } from 'styled-components'
import { ConnectionStatus } from '../../Screens/Main.screen'
import { Icon } from '../Icon'
import { IconType } from '../Icon/fa.defaults'
import { Spinner } from '../Spinner'

export const boardHeaderStyles = css`
	display: flex;
	align-items: center;
	min-width: 400px;

	& > .icon {
		height: 3rem;
		width: auto;
		margin-right: 1em;
	}

	& > h2 {
		padding: none;
		margin: none;
		font-size: 1.5rem;
		margin-right: 1.5rem;
	}

	.status-info {
		margin-left: auto;
		padding: 0.5em 1em;
		border: solid 1px ${props => props.theme.colors.lightGrey.val};
		background: ${props => props.theme.colors.black.opacity(40).val};
		border-radius: 5px;

		h5 {
			margin: 0;
			padding: 0;
			margin-bottom: 8px;
			color: ${props => props.theme.colors.grey.val};
			font-weight: normal;
		}
	}
	.status {
		display: flex;
		align-items: center;
		color: ${props => props.theme.colors.white.val};

		.msg {
			font-size: 0.9rem;
			display: flex;
			align-items: center;

			.spinner {
				margin-left: 0.5em;
				height: auto;
				span {
					& > span {
						height: 5px !important;
						width: 5px !important;
					}
				}
			}
		}

		&.__err {
			color: ${props => props.theme.colors.err.bright(2).val};
		}
		&.__ok {
			color: ${props => props.theme.colors.ok.bright(2).val};
		}
	}
`

const BoardHeaderView = styled.div`
	${boardHeaderStyles}
`

type ParsedStatus = {
	msg: string
	icon: IconType
}
const parseStatus = (status: ConnectionStatus): ParsedStatus =>
	status === 'OK'
		? {
				msg: 'Connected',
				icon: 'ok',
		  }
		: status === 'DROPPED'
		? {
				msg: 'Connection Dropped',
				icon: 'err',
		  }
		: status === 'RETRYING'
		? {
				msg: 'Reconnecting',
				icon: 'alert',
		  }
		: status === 'DISCONNECTED'
		? {
				msg: 'Ready',
				icon: 'circle',
		  }
		: {
				msg: 'Error Connecting',
				icon: 'err',
		  }

export type BoardHeaderProps = {
	status: ConnectionStatus | null
}
export const BoardHeader = ({ status }: BoardHeaderProps): JSX.Element => {
	const parsedStatus = status && parseStatus(status)
	const { msg, icon } = parsedStatus || {}

	const classStatus =
		status === 'OK' ? 'ok' : status === 'ERROR' || status === 'DROPPED' ? 'err' : 'generic'
	return (
		<BoardHeaderView>
			<Icon type='ssh' />
			<h2>ECLIPTIC SSH CLIENT</h2>
			<div className='status-info'>
				<h5>Status:</h5>

				<div className={`status __${classStatus}`}>
					<Icon type={icon || 'circle'} padRight />
					<div className='msg'>
						{msg}
						{status === 'RETRYING' ? (
							<Spinner type='dots' noBg isOverlay={false} height='sm' color='#fff' />
						) : null}
					</div>
				</div>
			</div>
		</BoardHeaderView>
	)
}
