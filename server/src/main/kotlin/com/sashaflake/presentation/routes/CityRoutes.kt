package com.sashaflake.presentation.routes

import com.sashaflake.domain.city.City
import com.sashaflake.infrastructure.persistence.CityRepository
import io.ktor.http.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.server.routing.*

fun Route.cityRoutes(cityRepository: CityRepository) {
    route("/cities") {
        post {
            val city = call.receive<City>()
            val id = cityRepository.create(city)
            call.respond(HttpStatusCode.Created, id)
        }
        get("{id}") {
            val id = call.parameters["id"]?.toIntOrNull()
                ?: return@get call.respond(HttpStatusCode.BadRequest, "Invalid ID")
            try {
                call.respond(HttpStatusCode.OK, cityRepository.read(id))
            } catch (e: NoSuchElementException) {
                call.respond(HttpStatusCode.NotFound)
            }
        }
        put("{id}") {
            val id = call.parameters["id"]?.toIntOrNull()
                ?: return@put call.respond(HttpStatusCode.BadRequest, "Invalid ID")
            cityRepository.update(id, call.receive<City>())
            call.respond(HttpStatusCode.OK)
        }
        delete("{id}") {
            val id = call.parameters["id"]?.toIntOrNull()
                ?: return@delete call.respond(HttpStatusCode.BadRequest, "Invalid ID")
            cityRepository.delete(id)
            call.respond(HttpStatusCode.OK)
        }
    }
}
