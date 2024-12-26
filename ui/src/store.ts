import {configureStore} from '@reduxjs/toolkit'

import sidebarReducer from './state/sidebar'
import settingsReducer from './state/settings'

export const store = configureStore({
    reducer: {
        sidebar: sidebarReducer,
        settings: settingsReducer
    },
})

// Infer the `RootState` and `AppDispatch` types from the store itself
export type RootState = ReturnType<typeof store.getState>
// Inferred type: {posts: PostsState, comments: CommentsState, users: UsersState}
export type AppDispatch = typeof store.dispatch