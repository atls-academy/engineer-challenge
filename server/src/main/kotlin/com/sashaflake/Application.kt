package com.sashaflake

import com.sashaflake.infrastructure.plugins.configureHTTP
import com.sashaflake.infrastructure.plugins.configureMonitoring
import com.sashaflake.infrastructure.plugins.configureSecurity
import com.sashaflake.infrastructure.plugins.configureSerialization
import com.sashaflake.presentation.configureRouting
import io.ktor.server.application.*

fun main(args: Array<String>) {
    io.ktor.server.netty.EngineMain.main(args)
}

fun Application.module() {
    configureHTTP()
    configureSecurity()
    configureMonitoring()
    configureSerialization()
    configureRouting()
}
