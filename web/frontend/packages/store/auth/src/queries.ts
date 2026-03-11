import { gql } from 'graphql-request'

export const LOGIN_MUTATION = gql`
  mutation Login($input: LoginInput!) {
    login(input: $input)
  }
`

export const REGISTER_MUTATION = gql`
  mutation Register($input: RegisterInput!) {
    register(input: $input)
  }
`

export const RECOVERY_MUTATION = gql`
  mutation Recovery($input: RecoveryInput!) {
    recovery(input: $input)
  }
`

export const UPDATE_SETTINGS_MUTATION = gql`
  mutation UpdateSettings($input: UpdateSettingsInput!) {
    updateSettings(input: $input)
  }
`