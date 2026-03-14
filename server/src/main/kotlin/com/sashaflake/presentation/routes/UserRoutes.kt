package com.sashaflake.presentation.routes

import com.sashaflake.domain.user.User
import com.sashaflake.infrastructure.persistence.UserRepository
import io.ktor.http.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.server.routing.*

fun Route.userRoutes(userRepository: UserRepository) {
    route("/users") {
        post {
            val user = call.receive<User>()
            val id = userRepository.create(user)
            call.respond(HttpStatusCode.Created, id)
        }
        get("{id}") {
            val id = call.parameters["id"]?.toIntOrNull()
                ?: return@get call.respond(HttpStatusCode.BadRequest, "Invalid ID")
            val user = userRepository.read(id)
                ?: return@get call.respond(HttpStatusCode.NotFound)
            call.respond(HttpStatusCode.OK, user)
        }
        put("{id}") {
            val id = call.parameters["id"]?.toIntOrNull()
                ?: return@put call.respond(HttpStatusCode.BadRequest, "Invalid ID")
            userRepository.update(id, call.receive<User>())
            call.respond(HttpStatusCode.OK)
        }
        delete("{id}") {
            val id = call.parameters["id"]?.toIntOrNull()
                ?: return@delete call.respond(HttpStatusCode.BadRequest, "Invalid ID")
            userRepository.delete(id)
            call.respond(HttpStatusCode.OK)
        }
    }
}
