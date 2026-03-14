rootProject.name = "engineer-challenge-impl"

pluginManagement {
    repositories {
        gradlePluginPortal()
        maven("https://redirector.kotlinlang.org/maven/kxrpc-grpc")
    }
}

dependencyResolutionManagement {
    repositories {
        mavenCentral()
        maven("https://redirector.kotlinlang.org/maven/kxrpc-grpc")
        maven("https://packages.confluent.io/maven/")
    }
}

include(":server")
include(":core")
include(":client")
