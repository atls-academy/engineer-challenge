'use client'
import React, { FC, useState, useEffect, useRef } from 'react'
import styles from './styles/index.module.scss'

export type Field = {
  name: string
  type: string
  placeholder: string
  title?: string
  error?: string
  required?: boolean
}

type PlaceholderFormProps = {
  fields: Field[]
  onSubmit?: (values: Record<string, string>) => void
  button?: React.ReactNode
}

const MIN_PASSWORD_LENGTH = 8

const validateField = (field: Field, value: string): string => {
  if (field.required && !value.trim()) {
    return 'Поле обязательно для заполнения'
  }
  if (field.type === 'email' && value) {
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
    if (!emailRegex.test(value)) return 'Недопустимый адрес почты'
  }
  if (field.type === 'password' && value) {
    if (value.length < MIN_PASSWORD_LENGTH)
      return `Минимум ${MIN_PASSWORD_LENGTH} символов`
  }
  return ''
}

const getInputType = (field: Field, isVisible: boolean): string => {
  if (field.type === 'password') {
    return isVisible ? 'text' : 'password'
  }
  return 'text'
}

export const PlaceholderForm: FC<PlaceholderFormProps> = ({
  fields,
  onSubmit,
  button,
}) => {
  const [values, setValues] = useState(
    fields.reduce(
      (acc, field) => {
        acc[field.name] = ''
        return acc
      },
      {} as Record<string, string>
    )
  )

  const [errors, setErrors] = useState<Record<string, string>>({})
  const [autofilled, setAutofilled] = useState<Record<string, boolean>>({})
  const [visibleFields, setVisibleFields] = useState<Record<string, boolean>>({})
  const formRef = useRef<HTMLFormElement>(null)

  useEffect(() => {
    const interval = setInterval(() => {
      if (!formRef.current) return
      const newAutofilled: Record<string, boolean> = {}
      fields.forEach((field) => {
        const el = formRef.current?.querySelector<HTMLInputElement>(`#${field.name}`)
        if (el) {
          try {
            newAutofilled[field.name] = el.matches(':-webkit-autofill')
          } catch {
            newAutofilled[field.name] = false
          }
        }
      })
      setAutofilled(newAutofilled)
    }, 300)
    return () => clearInterval(interval)
  }, [fields])

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target
    setValues((prev) => ({ ...prev, [name]: value }))
    if (errors[name]) {
      setErrors((prev) => ({ ...prev, [name]: '' }))
    }
  }

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    const newErrors: Record<string, string> = {}
    fields.forEach((field) => {
      const error = validateField(field, values[field.name])
      if (error) newErrors[field.name] = error
    })
    if (Object.keys(newErrors).length > 0) {
      setErrors(newErrors)
      return
    }
    if (onSubmit) onSubmit(values)
  }

  const toggleVisibility = (name: string) => {
    setVisibleFields((prev) => ({ ...prev, [name]: !prev[name] }))
  }

  return (
    <form
      ref={formRef}
      className={styles.authContainer}
      onSubmit={handleSubmit}
      noValidate
    >
      {fields.map((field) => (
        <div key={field.name} className={styles.fieldWrapper}>
          <div
            className={`${styles.inputWrapper} ${errors[field.name] ? styles.inputWrapperError : ''}`}
          >
            <input
              type={getInputType(field, visibleFields[field.name])}
              inputMode={field.type === 'email' ? 'email' : undefined}
              name={field.name}
              placeholder=" "
              value={values[field.name]}
              onChange={handleChange}
              autoComplete="off"
              className={`${styles.inputPlaceHolder} ${errors[field.name] ? styles.inputPlaceHolderError : ''}`}
              id={field.name}
            />
            {field.title && (
              <label
                htmlFor={field.name}
                className={`${styles.inputTitle} ${autofilled[field.name] ? styles.inputTitleActive : ''}`}
              >
                {field.title}
              </label>
            )}
            {field.type === 'password' && (
              <button
                type="button"
                className={styles.eyeButton}
                onClick={() => toggleVisibility(field.name)}
              >
                <img
                  src="https://cdn-icons-png.flaticon.com/128/2767/2767194.png"
                  alt="toggle visibility"
                  className={styles.eyeIcon}
                />
              </button>
            )}
          </div>
          {errors[field.name] && (
            <span className={styles.errorText}>{errors[field.name]}</span>
          )}
        </div>
      ))}

      {button && (
        <div className={styles.submitWrapper}>
          {button}
        </div>
      )}
    </form>
  )
}
