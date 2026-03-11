import { describe, it, expect, vi, beforeEach } from 'vitest'
import { configureStore } from '@reduxjs/toolkit'
import authReducer from '../slice'
import { loginThunk, registerThunk, recoveryThunk, updateSettingsThunk } from '../thunks'

const mockRequest = vi.fn()

vi.mock('../client', () => ({
  getClient: () => ({
    request: mockRequest,
  }),
}))

const makeStore = () => configureStore({ reducer: { auth: authReducer } })

const TEST_EMAIL = 'user@test.com'
const TEST_PASSWORD = 'test-password-mock'
const TEST_WRONG_PASSWORD = 'test-wrong-password-mock'
const TEST_NEW_PASSWORD = 'test-new-password-mock'

describe('thunks', () => {
  beforeEach(() => {
    mockRequest.mockReset()
  })

  it('loginThunk fulfilled sets isAuthenticated', async () => {
    mockRequest.mockResolvedValueOnce({ login: true })
    const store = makeStore()
    await store.dispatch(loginThunk({ identifier: TEST_EMAIL, password: TEST_PASSWORD }))
    expect(store.getState().auth.isAuthenticated).toBe(true)
  })

  it('loginThunk rejected sets error', async () => {
    mockRequest.mockRejectedValueOnce(new Error('Login failed'))
    const store = makeStore()
    await store.dispatch(loginThunk({ identifier: TEST_EMAIL, password: TEST_WRONG_PASSWORD }))
    expect(store.getState().auth.error).toBe('Login failed')
    expect(store.getState().auth.isAuthenticated).toBe(false)
  })

  it('registerThunk fulfilled sets isAuthenticated', async () => {
    mockRequest.mockResolvedValueOnce({ register: true })
    const store = makeStore()
    await store.dispatch(registerThunk({ email: TEST_EMAIL, password: TEST_PASSWORD }))
    expect(store.getState().auth.isAuthenticated).toBe(true)
  })

  it('recoveryThunk fulfilled does not set isAuthenticated', async () => {
    mockRequest.mockResolvedValueOnce({ recovery: true })
    const store = makeStore()
    await store.dispatch(recoveryThunk({ email: TEST_EMAIL }))
    expect(store.getState().auth.isAuthenticated).toBe(false)
    expect(store.getState().auth.error).toBeNull()
  })

  it('updateSettingsThunk fulfilled sets isAuthenticated', async () => {
    mockRequest.mockResolvedValueOnce({ updateSettings: 'success' })
    const store = makeStore()
    await store.dispatch(updateSettingsThunk({ method: 'password', password: TEST_NEW_PASSWORD }))
    expect(store.getState().auth.isAuthenticated).toBe(true)
  })
})
