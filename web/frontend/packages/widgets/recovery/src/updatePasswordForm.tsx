'use client'
import React, { FC } from 'react'
import { useDispatch, useSelector } from 'react-redux'
import { useRouter } from 'next/navigation'
import { PlaceholderForm, Field } from '@ui/placeholder'
import { PlaceholderTitle } from '@ui/placeholder-title'
import { Button } from '@ui/buttons'
import { updateSettingsThunk, selectAuthLoading, selectAuthError } from '@store/auth'
import type { AppDispatch } from '@store/root'
import styles from './styles/index.module.scss'

const fields: Field[] = [
  {
    name: 'password',
    type: 'password',
    placeholder: 'Введите пароль',
    title: 'Введите пароль',
    required: true,
  },
  {
    name: 'confirmPassword',
    type: 'password',
    placeholder: 'Повторите пароль',
    title: 'Повторите пароль',
    required: true,
  },
]

type UpdatePasswordFormProps = {
  onBack?: () => void
}

export const UpdatePasswordForm: FC<UpdatePasswordFormProps> = ({ onBack }) => {
  const dispatch = useDispatch<AppDispatch>()
  const loading = useSelector(selectAuthLoading)
  const error = useSelector(selectAuthError)
  const router = useRouter()

  const handleSubmit = async (values: Record<string, string>) => {
    if (values.password !== values.confirmPassword) {
      console.error('Пароли не совпадают')
      router.push('/settings-error')
      return
    }

    const result = await dispatch(
      updateSettingsThunk({ method: 'password', password: values.password })
    )

    if (updateSettingsThunk.fulfilled.match(result)) {
      router.push('/settings-success')
    } else {
      router.push('/settings-error')
    }
  }

  return (
    <div className={styles.loginContainer}>
      <div className={styles.titleWrapper}>
        <PlaceholderTitle
          text="Задайте пароль"
          subText="Напишите новый пароль который будете использовать для входа"
        />
      </div>
      {error && <span className={styles.errorText}>{error}</span>}
      <PlaceholderForm
        fields={fields}
        onSubmit={handleSubmit}
        button={<Button text={loading ? 'Загрузка...' : 'Изменить пароль'} />}
      />
    </div>
  )
}
