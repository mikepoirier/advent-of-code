package io.mikepoirier.adventofcode.processors.y2017

import io.mikepoirier.adventofcode.AnswerResponse
import io.mikepoirier.adventofcode.processors.DayProcessor
import org.slf4j.LoggerFactory
import org.springframework.stereotype.Component
import reactor.core.publisher.Flux
import reactor.core.publisher.Mono
import reactor.core.publisher.toFlux
import reactor.core.publisher.toMono
import reactor.core.scheduler.Schedulers

@Component
class DayTwoProcessor : DayProcessor<String> {

    private val log = LoggerFactory.getLogger(DayTwoProcessor::class.java)

    override fun match(year: String, day: String): Boolean {
        return year == "2017" && day == "2"
    }

    override fun handle(input: String): Mono<AnswerResponse> {
        return input.toMono()
            .doOnNext { log.info("Day two input: $it") }
            .map { it.trim() }
            .map { it.lines() }
            .flatMapMany {
                it.toFlux()
                    .publishOn(Schedulers.parallel())
                    .map { it.split('\t') }
                    .flatMap { strs ->
                        strs.toFlux()
                            .publishOn(Schedulers.parallel())
                            .map { it.toInt() }
                            .collectList()
                    }
            }
            .publishOn(Schedulers.parallel())
            .map {
                val max = it.max() ?: 0
                val min = it.min() ?: 0

                max - min
            }
            .reduce(0, { current, next -> current + next })
            .map { AnswerResponse(it) }
    }

}