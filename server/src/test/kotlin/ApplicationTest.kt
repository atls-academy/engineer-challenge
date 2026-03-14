package com.sashaflake

import com.example.proto.ClientGreeting
import com.example.proto.SampleService
import com.example.proto.invoke
import io.ktor.client.plugins.websocket.*
import io.ktor.client.request.*
import io.ktor.http.*
import io.ktor.server.testing.*
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.time.Duration.Companion.minutes
import kotlinx.rpc.grpc.client.GrpcClient
import kotlinx.rpc.grpc.ktor.server.grpc
import kotlinx.rpc.krpc.ktor.client.Krpc
import kotlinx.rpc.krpc.ktor.client.rpc
import kotlinx.rpc.krpc.ktor.client.rpcConfig
import kotlinx.rpc.krpc.serialization.json.json
import kotlinx.rpc.registerService
import kotlinx.rpc.withService

class ApplicationTest {

    @Test
    fun testRoot() = testApplication {
        application {
            module()
        }
        client.get("/").apply {
            assertEquals(HttpStatusCode.OK, status)
        }
    }

    @Test
    fun testRpc() = testApplication {
        application {
            configureFrameworks()
        }

        val ktorClient = createClient {
            install(WebSockets)
            install(Krpc)
        }

        val rpcClient = ktorClient.rpc("/api") {
            rpcConfig {
                serialization {
                    json()
                }
            }
        }

        val service = rpcClient.withService<SampleService>()

        val response = service.hello(Data("client"))

        assertEquals("Server: client", response)
    }

    @Test
    fun testRpc() = testApplication {
        application {
            grpc(8081) {
                services {
                    registerService<SampleService> { SampleServiceImpl() }
                }
            }
        }

        startApplication()

        val client = GrpcClient("localhost", 8081) {
            credentials = plaintext()
        }

        val response = client.withService<SampleService>().greeting(
            ClientGreeting {
                name = "Alex"
            }
        )

        assertEquals("Hello, Alex!", response.content, "Wrong response message")

        client.shutdown()
        client.awaitTermination(1.minutes)
    }

}
