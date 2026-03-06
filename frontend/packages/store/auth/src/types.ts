export interface LoginInput {
  identifier: string
  password: string
}

export interface RegisterInput {
  email: string
  password: string
  username?: string
}

export interface RecoveryInput {
  email: string
}

export interface AuthState {
  isAuthenticated: boolean
  loading: boolean
  error: string | null
}

export interface UpdateSettingsInput {
  method: string
  password?: string
  traits?: Record<string, unknown>
}
