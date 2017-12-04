package io.mikepoirier.adventofcode.processors.y2015

import io.mikepoirier.adventofcode.AnswerResponse
import io.mikepoirier.adventofcode.processors.DayProcessor
import org.springframework.stereotype.Component
import reactor.core.publisher.Flux
import reactor.core.publisher.Mono
import reactor.core.publisher.toFlux
import reactor.core.publisher.toMono

@Component
class DayOneProcessor2015 : DayProcessor<String> {
    override fun match(year: String, day: String): Boolean {
        return year == "2015" && day == "1"
    }

    override fun handle(input: String): Mono<AnswerResponse> {

        val toNumber: (Flux<String>) -> Flux<Int> = { flux ->
            flux.filter { it.isNotBlank() }
                .map {
                    when (it) {
                        "(" -> 1
                        ")" -> -1
                        else -> 0
                    }
                }
        }


        val partOneMono = input.toMono()
            .flatMapMany { it.split("").toFlux() }
            .transform(toNumber)
            .reduce { current, next -> current + next }

        val partTwoMono = input.toMono()
            .flatMapMany { it.split("").toFlux() }
            .transform(toNumber)
            .scan(0, { current, next ->
                current + next
            })
            .takeWhile { it > -1 }
            .count()

        return Mono.zip(partOneMono, partTwoMono)
            .map { AnswerResponse(it.t1, it.t2) }
    }
}