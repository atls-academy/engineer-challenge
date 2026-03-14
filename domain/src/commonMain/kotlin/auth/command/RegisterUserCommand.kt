package auth.command

data class RegisterUserCommand(
    val email: String,
    val password: String,
)
