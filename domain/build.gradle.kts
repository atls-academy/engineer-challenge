plugins {
    kotlin("multiplatform") version "2.3.0"
}

kotlin {
    jvm()

    sourceSets {
        commonMain.dependencies {
            // намеренно пусто — домен ни от чего не зависит
        }
    }
}
