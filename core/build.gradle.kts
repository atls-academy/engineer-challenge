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
            api("io.opentelemetry:opentelemetry-sdk-extension-autoconfigure:1.52.0")
            api("io.opentelemetry.semconv:opentelemetry-semconv:1.34.0")
            api("io.opentelemetry:opentelemetry-exporter-otlp:1.52.0")
            api("io.opentelemetry.instrumentation:opentelemetry-ktor-3.0:2.18.0-alpha")
            api("org.jetbrains.kotlin.plugin.serialization")
            api("org.jetbrains.kotlinx:kotlinx-rpc-grpc-core:$kotlinx_rpc_grpc_version")
            api("org.jetbrains.kotlinx:kotlinx-rpc-protobuf-core:$kotlinx_rpc_grpc_version")
        }
    }
}
