import { describe, it, expect } from 'vitest'
import authReducer, { resetError, logout } from '../slice'
import type { AuthState } from '../types'

const initialState: AuthState = {
  isAuthenticated: false,
  loading: false,
  error: null,
}

describe('authSlice', () => {
  it('returns initial state', () => {
    expect(authReducer(undefined, { type: '' })).toEqual(initialState)
  })

  it('resetError clears error', () => {
    const state = { ...initialState, error: 'some error' }
    expect(authReducer(state, resetError()).error).toBeNull()
  })

  it('logout sets isAuthenticated to false', () => {
    const state = { ...initialState, isAuthenticated: true }
    expect(authReducer(state, logout()).isAuthenticated).toBe(false)
  })

  it('loginThunk.pending sets loading true', () => {
    const action = { type: 'auth/login/pending' }
    const state = authReducer(initialState, action)
    expect(state.loading).toBe(true)
    expect(state.error).toBeNull()
  })

  it('loginThunk.fulfilled sets isAuthenticated true', () => {
    const action = { type: 'auth/login/fulfilled' }
    const state = authReducer({ ...initialState, loading: true }, action)
    expect(state.isAuthenticated).toBe(true)
    expect(state.loading).toBe(false)
  })

  it('loginThunk.rejected sets error', () => {
    const action = { type: 'auth/login/rejected', payload: 'Login failed' }
    const state = authReducer(initialState, action)
    expect(state.error).toBe('Login failed')
    expect(state.loading).toBe(false)
  })

  it('registerThunk.fulfilled sets isAuthenticated true', () => {
    const action = { type: 'auth/register/fulfilled' }
    const state = authReducer(initialState, action)
    expect(state.isAuthenticated).toBe(true)
  })

  it('recoveryThunk.fulfilled does not set isAuthenticated', () => {
    const action = { type: 'auth/recovery/fulfilled' }
    const state = authReducer(initialState, action)
    expect(state.isAuthenticated).toBe(false)
    expect(state.loading).toBe(false)
  })

  it('updateSettingsThunk.rejected sets error', () => {
    const action = { type: 'auth/updateSettings/rejected', payload: 'Update failed' }
    const state = authReducer(initialState, action)
    expect(state.error).toBe('Update failed')
  })
})
