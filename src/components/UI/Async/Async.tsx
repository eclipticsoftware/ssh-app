import {ReactNode} from 'react'
import styled from 'styled-components'
import {ErrorBlock} from '../ErrorBlock'
import Spinner from '../Spinner/Spinner'

export const AsyncView = styled.div``

export type AsyncProps = {
	fetchResults: {
		loading?: boolean
		fetching?: boolean
		error?: any
		[x: string]: any
	}
	children?: ReactNode
}
export const Async = ({ fetchResults, children }: AsyncProps) => {
	const { loading, fetching, error } = fetchResults
	return (
		<AsyncView>
			{loading || fetching ? <Spinner /> : error ? <ErrorBlock error={error} /> : children}
		</AsyncView>
	)
}
