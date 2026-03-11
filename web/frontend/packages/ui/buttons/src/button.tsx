import React from 'react'
import styles from './styles/index.module.scss'

export interface ButtonProps {
  text?: string
  design?: 'primary' | 'secondary'
  onClick?: () => void
  type?: 'button' | 'submit' | 'reset'
}

export const Button: React.FC<ButtonProps> = ({
  text = 'Кнопка',
  design = 'primary',
  onClick,
  type = 'submit',
}) => {
  return (
    <button
      type={type}
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
