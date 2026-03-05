'use client'
import React, { FC } from 'react'
import styles from './styles/index.module.scss'

type AuthFooterProps = {
  text: string
  linkText: string
  linkHref: string
}

export const AuthFooter: FC<AuthFooterProps> = ({
  text,
  linkText,
  linkHref,
}) => {
  return (
    <div className={styles.footer}>
      <span className={styles.text}>{text}</span>
      <a href={linkHref} className={styles.link}>
        {linkText}
      </a>
    </div>
  )
}
