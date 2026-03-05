'use client'
import React from 'react'
import { LoginForm } from '../../../widgets/login/src'
import { AuthFooter } from '@ui/auth-footer'
import { AuthDescText } from '@ui/auth-desc-text'
import styles from './styles/index.module.scss'

export const LoginPage = () => {
  return (
    <div className={styles.container}>
      <div className={styles.left}>
        <div className={styles.logo}>
          <img
            src="https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQ3LyAUdC0rlLZ1ADbJIQw9RCv23lFwgAJeFg&s"
            alt="logo"
            className={styles.logoImg}
          />
        </div>
        <div className={styles.formSide}>
          <LoginForm />
          <div className={styles.authDesc}>
            <AuthDescText linkText="Забыли пароль?" linkHref="/recovery" />
          </div>
        </div>
        <div className={styles.bottom}>
          <AuthFooter
            text="Ещё не зарегестрированны?"
            linkText="Зарегистрироваться"
            linkHref="/register"
          />
        </div>
      </div>
      <div className={styles.imageSide}>
        <img
          src="/wallpaper.jpg"
          alt="background"
          className={styles.imageSideImg}
        />
      </div>
    </div>
  )
}
