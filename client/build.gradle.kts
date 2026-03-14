val exposed_version: String by project
val h2_version: String by project
val koin_version: String by project
val kotlin_version: String by project
val kotlinx_rpc_grpc_version: String by project
val ktor_version: String by project
val logback_version: String by project
val postgres_version: String by project
val prometheus_version: String by project

plugins {
    kotlin("multiplatform") version "2.3.0"
    id("org.jetbrains.kotlin.plugin.serialization") version "2.3.0"
    id("org.jetbrains.kotlinx.rpc.plugin") version "0.11.0-grpc-185"
}

rpc {
    protoc()
}

kotlin {
    jvm()

    sourceSets {
        commonMain.dependencies {
            api(project(":core"))
            api("io.opentelemetry.instrumentation:opentelemetry-ktor-3.0:2.18.0-alpha")
            api("org.jetbrains.kotlinx:kotlinx-rpc-grpc-client:$kotlinx_rpc_grpc_version")
            api("io.grpc:grpc-netty:1.79.0")
            api("io.ktor:ktor-client-core:$ktor_version")
        }
    }
}
