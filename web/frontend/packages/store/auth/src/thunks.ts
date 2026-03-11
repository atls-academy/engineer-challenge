import { createAsyncThunk } from '@reduxjs/toolkit'
import { getClient } from './client'
import { LOGIN_MUTATION, REGISTER_MUTATION, RECOVERY_MUTATION, UPDATE_SETTINGS_MUTATION } from './queries'
import type { LoginInput, RegisterInput, RecoveryInput, UpdateSettingsInput } from './types'

export const loginThunk = createAsyncThunk(
  'auth/login',
  async (input: LoginInput, { rejectWithValue }) => {
    try {
      const client = getClient()
      const data = await client.request<{ login: boolean }>(LOGIN_MUTATION, { input })
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
      const data = await client.request<{ register: boolean }>(REGISTER_MUTATION, { input })
      return data.register
    } catch (error: unknown) {
      const message = error instanceof Error ? error.message : 'Registration failed'
      return rejectWithValue(message)
    }
  }
)

export const recoveryThunk = createAsyncThunk(
  'auth/recovery',
  async (input: RecoveryInput, { rejectWithValue }) => {
    try {
      const client = getClient()
      const data = await client.request<{ recovery: boolean }>(RECOVERY_MUTATION, { input })
      return data.recovery
    } catch (error: unknown) {
      const message = error instanceof Error ? error.message : 'Recovery failed'
      return rejectWithValue(message)
    }
  }
)

export const updateSettingsThunk = createAsyncThunk(
  'auth/updateSettings',
  async (input: UpdateSettingsInput, { rejectWithValue }) => {
    try {
      const client = getClient()
      const data = await client.request<{ updateSettings: string }>(UPDATE_SETTINGS_MUTATION, { input })
      return data.updateSettings
    } catch (error: unknown) {
      const message = error instanceof Error ? error.message : 'Update settings failed'
      return rejectWithValue(message)
    }
  }
)