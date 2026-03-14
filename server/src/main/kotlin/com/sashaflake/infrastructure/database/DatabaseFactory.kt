package com.sashaflake.infrastructure.database

import io.ktor.server.application.*
import java.sql.Connection
import java.sql.DriverManager

fun Application.connectToPostgres(embedded: Boolean = false): Connection {
    Class.forName("org.postgresql.Driver")
    return if (embedded) {
        log.info("Using embedded H2 database for testing")
        DriverManager.getConnection("jdbc:h2:mem:test;DB_CLOSE_DELAY=-1", "root", "")
    } else {
        val url = environment.config.property("postgres.url").getString()
        val user = environment.config.property("postgres.user").getString()
        val password = environment.config.property("postgres.password").getString()
        log.info("Connecting to postgres at $url")
        DriverManager.getConnection(url, user, password)
    }
}
