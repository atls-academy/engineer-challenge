package auth.command

import auth.model.user.Email
import auth.model.user.PlainPassword
import auth.model.user.User
import auth.model.user.UserId
import auth.port.PasswordHasher
import auth.port.UserRepository

class RegisterUserHandler(
    private val users: UserRepository,
    private val hasher: PasswordHasher,
) {
    fun handle(cmd: RegisterUserCommand): UserId {
        val email = Email.create(cmd.email)

        check(!users.existsByEmail(email)) {
            "User with this email already exists"
        }

        val user = User.register(
            email = email,
            plain = PlainPassword(cmd.password),
            hasher = hasher,
        )

        users.save(user)

        return user.id
    }
}
