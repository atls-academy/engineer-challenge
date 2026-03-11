'use client'
import React from 'react'
import { PlaceholderForm, Field } from '@ui/placeholder'
import { PlaceholderTitle } from '@ui/placeholder-title'
import { Button } from '@ui/buttons'
import { useAppDispatch, useAppSelector } from '@store/root'
import { registerThunk, selectAuthLoading, selectAuthError } from '@store/auth'
import styles from './styles/index.module.scss'

const fields: Field[] = [
  {
    name: 'email',
    type: 'email',
    placeholder: 'Введите email',
    title: 'Введите email',
  },
  {
    name: 'password',
    type: 'password',
    placeholder: 'Введите пароль',
    title: 'Введите пароль',
  },
  {
    name: 'confirmPassword',
    type: 'password',
    placeholder: 'Повторите пароль',
    title: 'Повторите пароль',
  },
]

export const RegisterForm = () => {
  const dispatch = useAppDispatch()
  const loading = useAppSelector(selectAuthLoading)
  const error = useAppSelector(selectAuthError)

  const handleSubmit = (values: Record<string, string>) => {
    if (values.password !== values.confirmPassword) return
    dispatch(
      registerThunk({
        email: values.email,
        password: values.password,
      })
    )
  }

  return (
    <div className={styles.loginContainer}>
      <div className={styles.titleWrapper}>
        <PlaceholderTitle text="Регистрация в системе" />
      </div>
      {error && <p className={styles.error}>{error}</p>}
      <PlaceholderForm
        fields={fields}
        onSubmit={handleSubmit}
        button={<Button text={loading ? 'Загрузка...' : 'Зарегистрироваться'} />}
      />
    </div>
  )
}
