package auth.model.user

import auth.port.PasswordHasher
import java.time.Instant

class User(
    val id: UserId,
    val email: Email,
    private var hashedPassword: HashedPassword,
    private var loginAttemptGuard: LoginAttemptGuard = LoginAttemptGuard.DEFAULT,
    val createdAt: Instant = Instant.now(),
) {
    fun canLogin(now: Instant): Boolean = !loginAttemptGuard.isLocked(now)

    fun verifyPassword(plain: PlainPassword, hasher: PasswordHasher, now: Instant): Boolean {
        if (!canLogin(now)) return false
        return if (hashedPassword.matches(plain, hasher)) {
            loginAttemptGuard = loginAttemptGuard.recordSuccess()
            true
        } else {
            loginAttemptGuard = loginAttemptGuard.recordFailure(now)
            false
        }
    }

    fun changePassword(plain: PlainPassword, hasher: PasswordHasher) {
        hashedPassword = HashedPassword.create(plain, hasher)
    }

    // For persistence layer only
    fun getHashedPassword(): HashedPassword = hashedPassword
    fun getLoginAttemptGuard(): LoginAttemptGuard = loginAttemptGuard

    companion object {
        fun register(email: Email, plain: PlainPassword, hasher: PasswordHasher): User =
            User(
                id = UserId.generate(),
                email = email,
                hashedPassword = HashedPassword.create(plain, hasher),
            )
    }
}
