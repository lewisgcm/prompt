import {createSlice} from '@reduxjs/toolkit'

export interface State {
    open: boolean
}

const initialState: State = {
    open: false,
}

export const slice = createSlice({
    name: 'settings',
    initialState,
    reducers: {
        open: (state) => {
            // Redux Toolkit allows us to write "mutating" logic in reducers. It
            // doesn't actually mutate the state because it uses the Immer library,
            // which detects changes to a "draft state" and produces a brand new
            // immutable state based off those changes
            state.open = true
        },
        close: (state) => {
            state.open = false
        },
    },
})

// Action creators are generated for each case reducer function
export const {open, close} = slice.actions

export default slice.reducer