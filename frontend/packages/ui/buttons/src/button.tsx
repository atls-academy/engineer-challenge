import React from 'react'
import styles from './styles/index.module.scss'

export interface ButtonProps {
  text?: string
  design?: 'primary' | 'secondary'
  onClick?: () => void
}

export const Button: React.FC<ButtonProps> = ({
  text = 'Кнопка',
  design = 'primary',
  onClick,
}) => {
  return (
    <button
      className={`${styles.button} ${design === 'secondary' ? styles.buttonSecondary : ''}`}
      onClick={onClick}
    >
      <span
        className={`${styles.buttonText} ${design === 'secondary' ? styles.buttonTextSecondary : ''}`}
      >
        {text}
      </span>
    </button>
  )
}
