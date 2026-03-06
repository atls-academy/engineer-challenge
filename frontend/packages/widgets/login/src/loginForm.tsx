'use client'
import React from 'react'
import { PlaceholderForm, Field } from '@ui/placeholder'
import { PlaceholderTitle } from '@ui/placeholder-title'
import { Button } from '@ui/buttons'
import { useAppDispatch, useAppSelector } from '@store/root'
import { loginThunk, selectAuthLoading, selectAuthError } from '@store/auth'
import styles from './styles/index.module.scss'

const fields: Field[] = [
  {
    name: 'email',
    type: 'email',
    placeholder: 'Введите e-mail',
    title: 'Введите e-mail',
  },
  {
    name: 'password',
    type: 'password',
    placeholder: 'Введите пароль',
    title: 'Введите пароль',
  },
]

export const LoginForm = () => {
  const dispatch = useAppDispatch()
  const loading = useAppSelector(selectAuthLoading)
  const error = useAppSelector(selectAuthError)

  const handleSubmit = (values: Record<string, string>) => {
    dispatch(loginThunk({ identifier: values.email, password: values.password }))
      .then((result) => {
        if (loginThunk.fulfilled.match(result)) {
          console.log('Login success')
        } else {
          console.log('Login failed:', result.payload)
        }
      })
  }

  return (
    <div className={styles.loginContainer}>
      <div className={styles.titleWrapper}>
        <PlaceholderTitle text="Войти в систему" />
      </div>
      {error && <p className={styles.error}>{error}</p>}
      <PlaceholderForm
        fields={fields}
        onSubmit={handleSubmit}
        button={<Button text={loading ? 'Загрузка...' : 'Войти'} />}
      />
    </div>
  )
}
