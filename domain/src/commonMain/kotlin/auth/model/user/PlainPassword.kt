package auth.model.user

@JvmInline
value class PlainPassword(val value: String) {
    // Never serialized, never logged, never persisted
    override fun toString(): String = "PlainPassword(***)"
}
