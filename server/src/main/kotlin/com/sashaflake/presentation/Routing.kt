package com.sashaflake.presentation

import com.sashaflake.infrastructure.plugins.appMicrometerRegistry
import com.sashaflake.presentation.routes.metricsRoutes
import io.ktor.server.application.*
import io.ktor.server.plugins.autohead.*
import io.ktor.server.routing.*

fun Application.configureRouting() {
    install(AutoHeadResponse)
    routing {
        metricsRoutes(appMicrometerRegistry)
    }
}
