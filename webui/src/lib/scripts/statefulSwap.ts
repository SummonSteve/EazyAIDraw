import {writable} from "svelte/store"

export default function statefulSwap(initialState) {
	const state = writable(initialState);
	let nextState = initialState;
	
	function transitionTo(newState) {
		if(nextState === newState) return;
		nextState = newState
		state.set(null)
	}
	
	function onOutro() {
		state.set(nextState)
	}
	return {
		state,
		transitionTo,
		onOutro
	}
}