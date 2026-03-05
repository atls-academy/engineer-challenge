'use client'
import React, { FC } from 'react'
import { PlaceholderTitle } from '@ui/placeholder-title'
import styles from './styles/index.module.scss'

type PasswordUpdateFailedFormProps = {
  onBack?: () => void
}

export const PasswordUpdateFailedForm: FC<PasswordUpdateFailedFormProps> = ({
  onBack,
}) => {
  return (
    <div className={styles.loginContainer}>
      <div className={styles.titleWrapper}>
        <PlaceholderTitle
          text="Пароль не был восстановлен"
          subText="По каким то причинам мы не смогли восстановить ваш пароль. Попробуйте ещё раз через некоторое время"
        />
      </div>
    </div>
  )
}
