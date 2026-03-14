val exposed_version: String by project
val h2_version: String by project
val koin_version: String by project
val kotlin_version: String by project
val kotlinx_rpc_grpc_version: String by project
val kotlinx_rpc_version: String by project
val logback_version: String by project
val postgres_version: String by project
val prometheus_version: String by project

plugins {
    kotlin("jvm") version "2.3.0"
    id("io.ktor.plugin") version "3.4.1"
    id("org.jetbrains.kotlin.plugin.serialization") version "2.3.0"
    id("org.jetbrains.kotlinx.rpc.plugin") version "0.10.2"
    id("org.jetbrains.kotlinx.rpc.plugin") version "0.11.0-grpc-185"
}

application {
    mainClass = "io.ktor.server.netty.EngineMain"
}

rpc {
    protoc()
}

kotlin {
    jvmToolchain(21)
}

dependencies {
    implementation(project(":core"))
    implementation("org.openfolder:kotlin-asyncapi-ktor:3.1.3")
    implementation("io.ktor:ktor-server-caching-headers")
    implementation("io.ktor:ktor-server-cors")
    implementation("io.ktor:ktor-server-http-redirect")
    implementation("io.ktor:ktor-server-forwarded-header")
    implementation("io.ktor:ktor-server-default-headers")
    implementation("io.ktor:ktor-server-core")
    implementation("io.ktor:ktor-server-openapi")
    implementation("io.ktor:ktor-server-routing-openapi")
    implementation("io.ktor:ktor-server-swagger")
    implementation("io.ktor:ktor-server-auth")
    implementation("io.ktor:ktor-server-csrf")
    implementation("io.ktor:ktor-server-auto-head-response")
    implementation("io.ktor:ktor-server-metrics-micrometer")
    implementation("io.micrometer:micrometer-registry-prometheus:$prometheus_version")
    implementation("io.ktor:ktor-server-call-logging")
    implementation("io.ktor:ktor-server-metrics")
    implementation("io.ktor:ktor-server-content-negotiation")
    implementation("io.ktor:ktor-serialization-kotlinx-json")
    implementation("org.jetbrains.exposed:exposed-core:$exposed_version")
    implementation("org.jetbrains.exposed:exposed-jdbc:$exposed_version")
    implementation("com.h2database:h2:$h2_version")
    implementation("org.postgresql:postgresql:$postgres_version")
    implementation("io.insert-koin:koin-ktor:$koin_version")
    implementation("io.insert-koin:koin-logger-slf4j:$koin_version")
    implementation("org.jetbrains.kotlinx:kotlinx-rpc-krpc-ktor-server:$kotlinx_rpc_version")
    implementation("org.jetbrains.kotlinx:kotlinx-rpc-krpc-ktor-client:$kotlinx_rpc_version")
    implementation("org.jetbrains.kotlinx:kotlinx-rpc-grpc-ktor-server:$kotlinx_rpc_grpc_version")
    implementation("org.jetbrains.kotlinx:kotlinx-rpc-grpc-client:$kotlinx_rpc_grpc_version")
    implementation("io.grpc:grpc-netty:1.79.0")
    implementation("io.github.flaxoos:ktor-server-rate-limiting:2.2.1")
    implementation("io.github.flaxoos:ktor-server-task-scheduling-core:2.2.1")
    implementation("io.github.flaxoos:ktor-server-task-scheduling-redis:2.2.1")
    implementation("io.github.flaxoos:ktor-server-task-scheduling-mongodb:2.2.1")
    implementation("io.github.flaxoos:ktor-server-task-scheduling-jdbc:2.2.1")
    implementation("io.ktor:ktor-server-netty")
    implementation("ch.qos.logback:logback-classic:$logback_version")
    implementation("io.ktor:ktor-server-config-yaml")
    testImplementation("io.ktor:ktor-server-test-host")
    testImplementation("org.jetbrains.kotlin:kotlin-test-junit:$kotlin_version")
}
