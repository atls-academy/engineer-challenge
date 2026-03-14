package auth.port

import auth.model.user.Email
import auth.model.user.User
import auth.model.user.UserId

interface UserRepository {
    fun findById(id: UserId): User?
    fun findByEmail(email: Email): User?
    fun save(user: User)
    fun existsByEmail(email: Email): Boolean
}
