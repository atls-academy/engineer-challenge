import { describe, it, expect } from 'vitest'
import { selectIsAuthenticated, selectAuthLoading, selectAuthError } from '../selectors'

const state = {
  auth: {
    isAuthenticated: true,
    loading: false,
    error: 'some error',
  },
}

describe('selectors', () => {
  it('selectIsAuthenticated', () => {
    expect(selectIsAuthenticated(state)).toBe(true)
  })

  it('selectAuthLoading', () => {
    expect(selectAuthLoading(state)).toBe(false)
  })

  it('selectAuthError', () => {
    expect(selectAuthError(state)).toBe('some error')
  })
})