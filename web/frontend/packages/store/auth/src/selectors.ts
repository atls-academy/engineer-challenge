import type { AuthState } from './types'

interface RootStateWithAuth {
  auth: AuthState
}

export const selectIsAuthenticated = (state: RootStateWithAuth) =>
  state.auth.isAuthenticated
export const selectAuthLoading = (state: RootStateWithAuth) =>
  state.auth.loading
export const selectAuthError = (state: RootStateWithAuth) => state.auth.error
