export interface LoginInput {
  identifier: string
  password: string
}

export interface RegisterInput {
  email: string
  password: string
  username?: string
}

export interface AuthState {
  isAuthenticated: boolean
  loading: boolean
  error: string | null
}
