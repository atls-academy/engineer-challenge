'use client'
import { RecoveryForm } from '../../../widgets/recovery/src'
import styles from './styles/index.module.scss'

type RecoveryPageProps = {
  onBack?: () => void
}

export const RecoveryPage = ({ onBack }: RecoveryPageProps) => {
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
        <RecoveryForm onBack={onBack} />
      </div>
    </div>
  )
}
