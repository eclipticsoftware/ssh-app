import React from 'react'

const useState = <T>(
	initialState: T,
	nameSpace: string = 'swfState'
): [T, React.Dispatch<React.SetStateAction<T>>] => {
	type StateObj = {
		[x: string]: T
	}

	const initialStateObj: StateObj = {}
	initialStateObj[nameSpace] = initialState
	const [state, setState] = React.useState<StateObj>(initialStateObj)

	const dispatch = ((incomingState: T) => {
		setState(st => {
			const newStateObj: StateObj = {}
			newStateObj[nameSpace] =
				typeof incomingState === 'function' ? incomingState(st[nameSpace]) : incomingState

			return newStateObj
		})
	}) as React.Dispatch<React.SetStateAction<T>>

	const currentState: T = state[nameSpace]

	return [currentState, dispatch]
}
export default useState
