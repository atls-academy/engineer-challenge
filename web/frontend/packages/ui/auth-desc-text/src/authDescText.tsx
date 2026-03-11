'use client'
import React, { FC } from 'react'
import styles from './styles/index.module.scss'

type AuthDescTextProps = {
  text?: string
  underlineWords?: string[]
  linkText?: string
  linkHref?: string
}

export const AuthDescText: FC<AuthDescTextProps> = ({
  text,
  underlineWords = [],
  linkText,
  linkHref,
}) => {
  const renderText = (str: string) => {
    if (!underlineWords.length) return str
    const regex = new RegExp(`(${underlineWords.join('|')})`, 'g')
    const parts = str.split(regex)
    return parts.map((part, i) =>
      underlineWords.includes(part) ? (
        <span key={`${part}-${i}`} className={styles.underline}>
          {part}
        </span>
      ) : (
        part
      )
    )
  }

  return (
    <div className={styles.wrapper}>
      {text && <span className={styles.text}>{renderText(text)}</span>}
      {linkText && linkHref && (
        <a href={linkHref} className={styles.link}>
          {linkText}
        </a>
      )}
    </div>
  )
}
