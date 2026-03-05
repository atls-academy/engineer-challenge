'use client'
import React, { FC } from 'react'
import { PlaceholderTitle } from '@ui/placeholder-title'
import styles from './styles/index.module.scss'

type PasswordUpdatedFormProps = {
  onBack?: () => void
}

export const PasswordUpdatedForm: FC<PasswordUpdatedFormProps> = ({
  onBack,
}) => {
  return (
    <div className={styles.loginContainer}>
      <div className={styles.titleWrapper}>
        <PlaceholderTitle
          text="Пароль был восстановлен"
          subText="Перейдите на страницу авторизации чтобы войти в систему с новым паролем"
        />
      </div>
    </div>
  )
}
