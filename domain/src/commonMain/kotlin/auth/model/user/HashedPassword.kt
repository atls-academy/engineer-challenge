package auth.model.user

import auth.port.PasswordHasher

@JvmInline
value class HashedPassword private constructor(val value: String) {

    companion object {
        private val POLICY = Regex("^(?=.*[A-Za-z])(?=.*\\d).{8,}$")

        fun create(plain: PlainPassword, hasher: PasswordHasher): HashedPassword {
            require(POLICY.matches(plain.value)) {
                "Password must be at least 8 characters and contain letters and digits"
            }
            return HashedPassword(hasher.hash(plain.value))
        }

        fun fromStorage(hash: String): HashedPassword = HashedPassword(hash)
    }

    fun matches(plain: PlainPassword, hasher: PasswordHasher): Boolean =
        hasher.verify(plain.value, this.value)
}
