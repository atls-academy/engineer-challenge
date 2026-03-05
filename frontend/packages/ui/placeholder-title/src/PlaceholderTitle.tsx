'use client'
import React, { FC } from 'react'
import styles from './styles/index.module.scss'

type PlaceholderTitleProps = {
  text: string
  subText?: string
  iconSrc?: string
  onIconClick?: () => void
}

export const PlaceholderTitle: FC<PlaceholderTitleProps> = ({
  text,
  subText,
  iconSrc,
  onIconClick,
}) => {
  return (
    <div>
      <div className={styles.titleRow}>
        {iconSrc && (
          <img
            src={iconSrc}
            alt=""
            className={`${styles.icon} ${onIconClick ? styles.iconClickable : ''}`}
            onClick={onIconClick}
          />
        )}
        <h1 className={styles.title}>{text}</h1>
      </div>
      {subText && <p className={styles.subText}>{subText}</p>}
    </div>
  )
}
