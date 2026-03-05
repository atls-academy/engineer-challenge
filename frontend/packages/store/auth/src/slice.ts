import { createSlice } from '@reduxjs/toolkit'
import type { AuthState } from './types'
import { loginThunk, registerThunk } from './thunks'

const initialState: AuthState = {
  isAuthenticated: false,
  loading: false,
  error: null,
}

const authSlice = createSlice({
  name: 'auth',
  initialState,
  reducers: {
    resetError(state) {
      state.error = null
    },
    logout(state) {
      state.isAuthenticated = false
      state.error = null
    },
  },
  extraReducers: (builder) => {
    builder
      .addCase(loginThunk.pending, (state) => {
        state.loading = true
        state.error = null
      })
      .addCase(loginThunk.fulfilled, (state) => {
        state.loading = false
        state.isAuthenticated = true
      })
      .addCase(loginThunk.rejected, (state, action) => {
        state.loading = false
        state.error = action.payload as string
      })

      .addCase(registerThunk.pending, (state) => {
        state.loading = true
        state.error = null
      })
      .addCase(registerThunk.fulfilled, (state) => {
        state.loading = false
        state.isAuthenticated = true
      })
      .addCase(registerThunk.rejected, (state, action) => {
        state.loading = false
        state.error = action.payload as string
      })
  },
})

export const { resetError, logout } = authSlice.actions
export default authSlice.reducer
