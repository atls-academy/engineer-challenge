package com.sashaflake.presentation

import com.sashaflake.infrastructure.database.connectToPostgres
import com.sashaflake.infrastructure.persistence.CityRepository
import com.sashaflake.infrastructure.persistence.UserRepository
import com.sashaflake.infrastructure.plugins.appMicrometerRegistry
import com.sashaflake.presentation.routes.cityRoutes
import com.sashaflake.presentation.routes.metricsRoutes
import com.sashaflake.presentation.routes.userRoutes
import io.ktor.server.application.*
import io.ktor.server.plugins.autohead.*
import io.ktor.server.routing.*
import org.jetbrains.exposed.sql.Database

fun Application.configureRouting() {
    install(AutoHeadResponse)

    val pgConnection = connectToPostgres(embedded = true)
    val cityRepository = CityRepository(pgConnection)

    val exposedDb = Database.connect(
        url = "jdbc:h2:mem:test;DB_CLOSE_DELAY=-1",
        driver = "org.h2.Driver",
        user = "root",
        password = ""
    )
    val userRepository = UserRepository(exposedDb)

    routing {
        cityRoutes(cityRepository)
        userRoutes(userRepository)
        metricsRoutes(appMicrometerRegistry)
    }
}
