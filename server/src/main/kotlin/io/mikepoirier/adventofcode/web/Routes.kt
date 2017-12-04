package io.mikepoirier.adventofcode.web

import org.springframework.context.annotation.Bean
import org.springframework.context.annotation.Configuration
import org.springframework.core.io.ClassPathResource
import org.springframework.http.MediaType
import org.springframework.web.reactive.function.server.router

private const val ROOT_PATH = "/"
private const val API_PATH = "/api"
private const val ANSWER_PATH = "/answer"
private const val RESOURCE_PATH = "/**"
private const val PUBLIC_RESOURCES = "/public"

@Configuration
class Routes(val answerHandler: AnswerHandler) {

    @Bean
    fun apiRoutes() = router {
        (API_PATH and accept(MediaType.APPLICATION_JSON)).nest {
            ANSWER_PATH.nest {
                GET(ROOT_PATH, answerHandler::handleRequest)
            }
        }
    }

    @Bean
    fun uiRoutes() = router {
        GET(ROOT_PATH).nest {
            resources(RESOURCE_PATH, ClassPathResource(PUBLIC_RESOURCES))
        }
    }
}