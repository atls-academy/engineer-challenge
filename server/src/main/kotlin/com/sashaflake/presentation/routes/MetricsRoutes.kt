package com.sashaflake.presentation.routes

import io.ktor.server.response.*
import io.ktor.server.routing.*
import io.micrometer.prometheus.PrometheusMeterRegistry

fun Route.metricsRoutes(registry: PrometheusMeterRegistry) {
    get("/metrics") {
        call.respond(registry.scrape())
    }
}
