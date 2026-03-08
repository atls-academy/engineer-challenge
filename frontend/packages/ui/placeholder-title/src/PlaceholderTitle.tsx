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
          onIconClick ? (
            <button
              type="button"
              role="button"
              onClick={onIconClick}
              onKeyDown={(e) => e.key === 'Enter' && onIconClick()}
              className={`${styles.icon} ${styles.iconClickable}`}
            >
              <img src={iconSrc} alt="" />
            </button>
          ) : (
            <img src={iconSrc} alt="" className={styles.icon} />
          )
        )}
        <h1 className={styles.title}>{text}</h1>
      </div>
      {subText && <p className={styles.subText}>{subText}</p>}
    </div>
  )
}
