'use client'
import { RegisterForm } from '@widgets/registration'
import { AuthFooter } from '@ui/auth-footer'
import { AuthDescText } from '@ui/auth-desc-text'
import styles from './styles/index.module.scss'

export const RegisterPage = () => {
  return (
    <div className={styles.container}>
      <div className={styles.left}>
        <div className={styles.logo}>
          <img
            src="https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQ3LyAUdC0rlLZ1ADbJIQw9RCv23lFwgAJeFg&s"
            alt="logo"
            className={styles.logoImg}
          />
        </div>
        <div className={styles.formSide}>
          <RegisterForm />
          <div className={styles.authDesc}>
            <AuthDescText
              text="Зарегистрировавшись пользователь принимает условия договора оферты и политики конфиденциальности"
              underlineWords={[
                'договора оферты',
                'политики конфиденциальности',
              ]}
            />
          </div>
        </div>
        <div className={styles.bottom}>
          <AuthFooter
            text="Уже есть аккаунт?"
            linkText="Войти"
            linkHref="/login"
          />
        </div>
      </div>
      <div className={styles.imageSide}>
        <img
          src="/wallpaper.jpg"
          alt="background"
          className={styles.imageSideImg}
        />
      </div>
    </div>
  )
}
