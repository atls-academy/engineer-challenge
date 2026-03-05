import { createAsyncThunk } from '@reduxjs/toolkit'
import { getClient } from './client'
import { LOGIN_MUTATION, REGISTER_MUTATION } from './queries'
import type { LoginInput, RegisterInput } from './types'

export const loginThunk = createAsyncThunk(
  'auth/login',
  async (input: LoginInput, { rejectWithValue }) => {
    try {
      const client = getClient()
      const data = await client.request<{ login: boolean }>(LOGIN_MUTATION, {
        input,
      })
      return data.login
    } catch (error: unknown) {
      const message = error instanceof Error ? error.message : 'Login failed'
      return rejectWithValue(message)
    }
  }
)

export const registerThunk = createAsyncThunk(
  'auth/register',
  async (input: RegisterInput, { rejectWithValue }) => {
    try {
      const client = getClient()
      const data = await client.request<{ register: boolean }>(
        REGISTER_MUTATION,
        { input }
      )
      return data.register
    } catch (error: unknown) {
      const message =
        error instanceof Error ? error.message : 'Registration failed'
      return rejectWithValue(message)
    }
  }
)
