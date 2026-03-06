'use client'
import React, { FC } from 'react'
import { useDispatch, useSelector } from 'react-redux'
import { PlaceholderForm, Field } from '@ui/placeholder'
import { PlaceholderTitle } from '@ui/placeholder-title'
import { Button } from '@ui/buttons'
import { recoveryThunk } from '@store/auth'
import { selectAuthLoading, selectAuthError } from '@store/auth'
import type { AppDispatch } from '@store/root'
import styles from './styles/index.module.scss'

const fields: Field[] = [
  {
    name: 'email',
    type: 'email',
    placeholder: 'Введите email',
    title: 'Введите email',
    required: true,
  },
]

type RecoveryFormProps = {
  onBack?: () => void
}

export const RecoveryForm: FC<RecoveryFormProps> = ({ onBack }) => {
  const dispatch = useDispatch<AppDispatch>()
  const loading = useSelector(selectAuthLoading)
  const error = useSelector(selectAuthError)

  const handleSubmit = (values: Record<string, string>) => {
    dispatch(recoveryThunk({ email: values.email }))
  }

  return (
    <div className={styles.loginContainer}>
      <div className={styles.titleWrapper}>
        <PlaceholderTitle
          text="Восстановление пароля"
          subText="Укажите адрес почты на который был зарегистрирован аккаунт"
          iconSrc="https://cdn-icons-png.flaticon.com/128/271/271220.png"
          onIconClick={onBack}
        />
      </div>
      {error && <span style={{ color: 'red', fontSize: '14px' }}>{error}</span>}
      <PlaceholderForm
        fields={fields}
        onSubmit={handleSubmit}
        button={<Button text={loading ? 'Загрузка...' : 'Восстановить'} design="secondary" />}
      />
    </div>
  )
}