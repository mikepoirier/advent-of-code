package io.mikepoirier.adventofcode.web

import io.mikepoirier.adventofcode.AnswerResponse
import io.mikepoirier.adventofcode.config.ApplicationProperties
import io.mikepoirier.adventofcode.processors.DayProcessor
import org.slf4j.LoggerFactory
import org.springframework.http.HttpStatus
import org.springframework.http.MediaType
import org.springframework.stereotype.Component
import org.springframework.web.reactive.function.client.WebClient
import org.springframework.web.reactive.function.client.bodyToMono
import org.springframework.web.reactive.function.server.ServerRequest
import org.springframework.web.reactive.function.server.ServerResponse
import org.springframework.web.reactive.function.server.body
import reactor.core.publisher.Flux
import reactor.core.publisher.Mono
import reactor.core.publisher.toFlux
import reactor.core.publisher.toMono
import reactor.core.scheduler.Schedulers

@Component
class AnswerHandler(
    val applicationProperties: ApplicationProperties,
    val dayProcessors: List<DayProcessor<String>>
) {

    private val log = LoggerFactory.getLogger(AnswerHandler::class.java)

    fun handleRequest(req: ServerRequest): Mono<ServerResponse> {
        val processorsFlux = dayProcessors.toFlux()
            .publishOn(Schedulers.parallel())
        val yearMono = Mono.justOrEmpty(req.queryParam("year"))
        val dayMono = Mono.justOrEmpty(req.queryParam("day"))

        val notFound = { httpStatus: HttpStatus? -> httpStatus?.value() == 404 }

        val yearDayMono = Mono.zip(yearMono, dayMono)

        // TODO: Don't make the http request if there are not handlers for the given day
        val inputMono = yearDayMono
            .flatMap {
                val year = it.t1
                val day = it.t2

                WebClient.create(applicationProperties.host)
                    .get()
                    .uri("/$year/day/$day/input")
                    .accept(MediaType.TEXT_PLAIN)
                    .header("Cookie", "session=${applicationProperties.session}")
                    .retrieve()
                    .onStatus(notFound, { _ -> Mono.error(NotFoundException()) })
                    .bodyToMono<String>()
                    .onErrorResume { Mono.just("Not Found") }
            }

        val processorMono = yearDayMono
            .flatMap { yearDayTuple ->
                val year = yearDayTuple.t1
                val day = yearDayTuple.t2
                processorsFlux.filter { it.match(year, day) }
                    .single()
                    .onErrorResume { _ -> Mono.empty() }
            }

        val responseMono = Mono.zip(processorMono, inputMono)
            .flatMap { it.t1.handle(it.t2) }
            .defaultIfEmpty(AnswerResponse())

        return ServerResponse.ok().json().body(responseMono)
    }
}

sealed class Option<out A> {
    object None : Option<Nothing>()
    data class Some<out A>(val item: A) : Option<A>()
}

class NotFoundException : Exception()