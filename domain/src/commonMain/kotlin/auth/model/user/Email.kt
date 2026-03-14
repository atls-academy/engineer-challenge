package auth.model.user

@JvmInline
value class Email private constructor(val value: String) {

    companion object {
        private val PATTERN = Regex("^[^@\\s]+@[^@\\s]+\\.[^@\\s]+$")

        fun create(raw: String): Email {
            val normalized = raw.trim().lowercase()
            require(PATTERN.matches(normalized)) { "Invalid email format: $raw" }
            return Email(normalized)
        }

        fun fromStorage(value: String): Email = Email(value)
    }

    override fun toString(): String = value
}
