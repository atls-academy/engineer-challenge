'use client'
import { useRouter } from 'next/navigation'
import { PasswordUpdatedForm } from '../../../widgets/recovery/src'
import { Button } from '@ui/buttons'
import styles from './styles/index.module.scss'

export const PasswordUpdatedPage = () => {
  const router = useRouter()

  return (
    <div className={styles.container}>
      <div className={styles.logo}>
        <img
          src="https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQ3LyAUdC0rlLZ1ADbJIQw9RCv23lFwgAJeFg&s"
          alt="logo"
          className={styles.logoImg}
        />
      </div>
      <div className={styles.formSide}>
        <PasswordUpdatedForm onBack={() => router.back()} />
        <Button
          text="Назад в авторизацию"
          design="secondary"
          onClick={() => router.push('/login')}
        />
      </div>
    </div>
  )
}
