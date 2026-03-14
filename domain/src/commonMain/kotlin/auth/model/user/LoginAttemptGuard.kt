package auth.model.user

import java.time.Duration
import java.time.Instant

data class LoginAttemptGuard(
    val failedAttempts: Int = 0,
    val lockedUntil: Instant? = null,
) {
    fun isLocked(now: Instant): Boolean =
        lockedUntil != null && now.isBefore(lockedUntil)

    fun recordFailure(now: Instant): LoginAttemptGuard {
        val next = failedAttempts + 1
        return if (next >= MAX_ATTEMPTS) {
            copy(failedAttempts = next, lockedUntil = now.plus(LOCK_DURATION))
        } else {
            copy(failedAttempts = next)
        }
    }

    fun recordSuccess(): LoginAttemptGuard = DEFAULT

    companion object {
        val DEFAULT = LoginAttemptGuard()
        const val MAX_ATTEMPTS = 5
        val LOCK_DURATION: Duration = Duration.ofMinutes(15)
    }
}
