'use client'
import React, { FC } from 'react'
import { PlaceholderForm, Field } from '@ui/placeholder'
import { PlaceholderTitle } from '@ui/placeholder-title'
import { Button } from '@ui/buttons'
import styles from './styles/index.module.scss'

const fields: Field[] = [
  {
    name: 'email',
    type: 'email',
    placeholder: 'Введите email',
    title: 'Введите email',
  },
]

const handleSubmit = (values: Record<string, string>) => {
  console.log('Форма отправлена:', values)
}

type RecoveryFormProps = {
  onBack?: () => void
}

export const RecoveryForm: FC<RecoveryFormProps> = ({ onBack }) => {
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
      <PlaceholderForm
        fields={fields}
        onSubmit={handleSubmit}
        button={<Button text="Восстановить" design="secondary" />}
      />
    </div>
  )
}
