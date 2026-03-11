'use client'
import React, { FC } from 'react'
import { PlaceholderTitle } from '@ui/placeholder-title'
import styles from './styles/index.module.scss'

type CheckMailFormProps = {
  onBack?: () => void
}

export const CheckMailForm: FC<CheckMailFormProps> = ({ onBack }) => {
  return (
    <div className={styles.loginContainer}>
      <div className={styles.titleWrapper}>
        <PlaceholderTitle
          text="Проверьте свою почту"
          subText="Мы отправили на почту письмо с ссылкой для восстановления пароля"
        />
      </div>
    </div>
  )
}
