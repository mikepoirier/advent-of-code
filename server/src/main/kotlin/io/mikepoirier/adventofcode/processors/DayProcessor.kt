package io.mikepoirier.adventofcode.processors

import io.mikepoirier.adventofcode.AnswerResponse
import org.springframework.stereotype.Component
import reactor.core.publisher.Mono

interface DayProcessor<in T> {
    fun match(year: String, day: String): Boolean
    fun handle(input: T): Mono<AnswerResponse>
}