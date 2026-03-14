package auth.port

interface PasswordHasher {
    fun hash(plain: String): String
    fun verify(plain: String, hash: String): Boolean
}
