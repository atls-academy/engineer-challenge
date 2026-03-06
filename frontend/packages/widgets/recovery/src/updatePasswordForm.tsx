'use client'
import React, { FC } from 'react'
import { PlaceholderForm, Field } from '@ui/placeholder'
import { PlaceholderTitle } from '@ui/placeholder-title'
import { Button } from '@ui/buttons'
import styles from './styles/index.module.scss'

const fields: Field[] = [
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

const handleSubmit = (values: Record<string, string>) => {
  console.log('Форма отправлена:', values)
}

type UpdatePasswordFormProps = {
  onBack?: () => void
}

export const UpdatePasswordForm: FC<UpdatePasswordFormProps> = ({ onBack }) => {
  return (
    <div className={styles.loginContainer}>
      <div className={styles.titleWrapper}>
        <PlaceholderTitle
          text="Задайте пароль"
          subText="Напишите новый пароль который будете использовать для входа"
        />
      </div>
      <PlaceholderForm
        fields={fields}
        onSubmit={handleSubmit}
        button={<Button text="Изменить пароль" />}
      />
    </div>
  )
}
