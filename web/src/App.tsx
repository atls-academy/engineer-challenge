import { useState, useEffect } from 'react'
import './App.css'
import { authClient } from './connect-client'
import logoImg from './branding/logo.png'
import decorImg from './branding/image.png'

type View = 'login' | 'register' | 'forgot' | 'dashboard' | 'reset'

const EyeIcon = () => (
    <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
        <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"></path>
        <circle cx="12" cy="12" r="3"></circle>
    </svg>
)

const EyeOffIcon = () => (
    <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
        <path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"></path>
        <line x1="1" y1="1" x2="23" y2="23"></line>
    </svg>
)

function App() {
    const [view, setView] = useState<View>('login')
    const [token, setToken] = useState('')
    const [email, setEmail] = useState('')
    const [password, setPassword] = useState('')
    const [confirmPassword, setConfirmPassword] = useState('')
    const [loading, setLoading] = useState(false)
    const [message, setMessage] = useState('')
    const [error, setError] = useState('')

    const [showPassword, setShowPassword] = useState(false)
    const [showConfirmPassword, setShowConfirmPassword] = useState(false)

    // Check for reset password route on load
    useEffect(() => {
        const path = window.location.pathname
        if (path === '/reset-password') {
            const params = new URLSearchParams(window.location.search)
            const t = params.get('token')
            if (t) {
                setToken(t)
                setView('reset')
            } else {
                setError('Токен отсутствует')
                setView('login')
            }
        }
    }, [])

    // Handlers
    const handleLogin = async (e: React.FormEvent) => {
        e.preventDefault()
        setLoading(true)
        setError('')
        setMessage('')
        try {
            const resp = await authClient.login({ email, password })
            setMessage('Successfully logged in!')
            console.log('Login success:', resp)
            setView('dashboard')
        } catch (err: any) {
            setError(err.message || 'Login failed')
        } finally {
            setLoading(false)
        }
    }

    const handleRegister = async (e: React.FormEvent) => {
        e.preventDefault()
        if (password !== confirmPassword) {
            setError('Пароли не совпадают')
            return
        }

        setLoading(true)
        setError('')
        setMessage('')
        try {
            await authClient.register({ email, password })
            setMessage('Registration successful! Please login.')
            setView('login')
        } catch (err: any) {
            setError(err.message || 'Registration failed')
        } finally {
            setLoading(false)
        }
    }

    const handleForgot = async (e: React.FormEvent) => {
        e.preventDefault()
        setLoading(true)
        setError('')
        setMessage('')
        try {
            await authClient.initiatePasswordReset({ email })
            setMessage('Если аккаунт существует, ссылка была отправлена.')
        } catch (err: any) {
            setError(err.message || 'Ошибка запроса')
        } finally {
            setLoading(false)
        }
    }

    const handleReset = async (e: React.FormEvent) => {
        e.preventDefault()
        if (password !== confirmPassword) {
            setError('Пароли не совпадают')
            return
        }

        setLoading(true)
        setError('')
        setMessage('')
        try {
            await authClient.completePasswordReset({ token, newPassword: password })
            setMessage('Пароль успешно изменен! Теперь вы можете войти.')
            setTimeout(() => setView('login'), 2000)
        } catch (err: any) {
            setError(err.message || 'Ошибка сброса пароля')
        } finally {
            setLoading(false)
        }
    }

    return (
        <div className="container">
            <div className="login-side">
                <header className="logo">
                    <img src={logoImg} alt="Orbitto Logo" className="logo-img" />
                </header>

                <main className="form-container">
                    <h1>
                        {view === 'login' ? 'Войти в систему' :
                            view === 'register' ? 'Регистрация' :
                                view === 'dashboard' ? 'Добро пожаловать!' :
                                    view === 'reset' ? 'Новый пароль' :
                                        'Сброс пароля'}
                    </h1>

                    {view === 'dashboard' ? (
                        <div className="dashboard-content">
                            <p>Вы успешно авторизовались в закрытой части приложения.</p>
                            <button className="login-btn" onClick={() => setView('login')}>
                                Выйти
                            </button>
                        </div>
                    ) : (
                        <form onSubmit={
                            view === 'login' ? handleLogin :
                                view === 'register' ? handleRegister :
                                    view === 'reset' ? handleReset :
                                        handleForgot
                        }>
                            {view !== 'reset' && (
                                <div className="input-group">
                                    <label>Введите e-mail</label>
                                    <div className="input-wrapper">
                                        <input
                                            type="email"
                                            value={email}
                                            onChange={(e) => setEmail(e.target.value)}
                                            placeholder="e-mail"
                                            className={error && (error.toLowerCase().includes('email') || error.toLowerCase().includes('login') || error.toLowerCase().includes('failed') || error.toLowerCase().includes('account')) ? 'error' : ''}
                                            required
                                        />
                                    </div>
                                </div>
                            )}

                            {view !== 'forgot' && (
                                <>
                                    <div className="input-group">
                                        <label>Введите пароль</label>
                                        <div className="input-wrapper">
                                            <input
                                                type={showPassword ? "text" : "password"}
                                                value={password}
                                                onChange={(e) => setPassword(e.target.value)}
                                                placeholder="пароль"
                                                className={error && (error.toLowerCase().includes('парол') || error.toLowerCase().includes('login')) ? 'error' : ''}
                                                required
                                            />
                                            <button
                                                type="button"
                                                className="password-toggle"
                                                onClick={() => setShowPassword(!showPassword)}
                                            >
                                                {showPassword ? <EyeOffIcon /> : <EyeIcon />}
                                            </button>
                                        </div>
                                        {error && error.toLowerCase().includes('пароль должен') && <div className="status-message error">{error}</div>}
                                    </div>
                                    {(view === 'register' || view === 'reset') && (
                                        <div className="input-group">
                                            <label>Повторите пароль</label>
                                            <div className="input-wrapper">
                                                <input
                                                    type={showConfirmPassword ? "text" : "password"}
                                                    value={confirmPassword}
                                                    onChange={(e) => setConfirmPassword(e.target.value)}
                                                    placeholder="повторите пароль"
                                                    className={error && error.includes('совпадают') ? 'error' : ''}
                                                    required
                                                />
                                                <button
                                                    type="button"
                                                    className="password-toggle"
                                                    onClick={() => setShowConfirmPassword(!showConfirmPassword)}
                                                >
                                                    {showConfirmPassword ? <EyeOffIcon /> : <EyeIcon />}
                                                </button>
                                            </div>
                                            {error && error.includes('совпадают') && <div className="status-message error">{error}</div>}
                                        </div>
                                    )}
                                </>
                            )}

                            {error && !error.toLowerCase().includes('содержать') && !error.includes('совпадают') && (
                                <div className="form-error">{error}</div>
                            )}
                            {message && <div className="form-success">{message}</div>}

                            <button className="login-btn" disabled={loading}>
                                {loading ? 'Загрузка...' :
                                    view === 'login' ? 'Войти' :
                                        view === 'register' ? 'Создать аккаунт' :
                                            view === 'reset' ? 'Сменить пароль' :
                                                'Сбросить'}
                            </button>

                            {view === 'login' && (
                                <button type="button" className="forgot-link" onClick={() => setView('forgot')}>
                                    Забыли пароль?
                                </button>
                            )}
                        </form>
                    )}
                </main>

                <footer className="footer">
                    {view === 'dashboard' ? null : view === 'login' ? (
                        <>Еще не зарегистрированы? <button className="link" onClick={() => setView('register')}>Регистрация</button></>
                    ) : (
                        <>Уже есть аккаунт? <button className="link" onClick={() => setView('login')}>Войти</button></>
                    )}
                </footer>
            </div>

            <div className="decoration-side">
                <img src={decorImg} alt="Orbitto Decoration" className="decoration-image" />
            </div>
        </div>
    )
}

export default App
