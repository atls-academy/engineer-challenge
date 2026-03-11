'use client'
import { UpdatePasswordForm } from '@widgets/recovery'
import styles from './styles/index.module.scss'

type UpdatePasswordPageProps = {
  onBack?: () => void
}

export const UpdatePasswordPage = ({ onBack }: UpdatePasswordPageProps) => {
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
        <UpdatePasswordForm onBack={onBack} />
      </div>
    </div>
  )
}
