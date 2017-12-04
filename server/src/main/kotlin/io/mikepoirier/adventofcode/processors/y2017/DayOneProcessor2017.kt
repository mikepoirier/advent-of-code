package io.mikepoirier.adventofcode.processors.y2017

import io.mikepoirier.adventofcode.AnswerResponse
import io.mikepoirier.adventofcode.processors.DayProcessor
import io.mikepoirier.adventofcode.web.Option
import org.slf4j.LoggerFactory
import org.springframework.stereotype.Component
import reactor.core.publisher.Mono
import reactor.core.publisher.toFlux
import reactor.core.publisher.toMono
import reactor.util.function.Tuples

@Component
class DayOneProcessor2017 : DayProcessor<String> {
    private val log = LoggerFactory.getLogger(DayOneProcessor2017::class.java)

    override fun match(year: String, day: String): Boolean {
        return year == "2017" && day == "1"
    }

    override fun handle(input: String): Mono<AnswerResponse> {
        return input.toMono()
            .doOnNext { log.info("Day one input: $it") }
            .map { it.trim() }
            .flatMapMany { it.split("").toFlux() }
            .filter { it.isNotBlank() }
            .map { it.toInt() }
            .collectList()
            .map {
                val initialOption: Option<Int> = Option.None
                val fold = it.fold(Tuples.of(0, initialOption), { current, next ->
                    val currentSum = current.t1
                    val currentInt = current.t2
                    val sum = when (currentInt) {
                        is Option.None -> currentSum
                        is Option.Some -> {
                            if (currentInt.item == next) {
                                currentSum + next
                            } else {
                                currentSum
                            }
                        }
                    }
                    Tuples.of(sum, Option.Some(next))
                })
                if (it.first() == it.last()) {
                    fold.t1 + it.first()
                } else {
                    fold.t1
                }
            }
            .map { AnswerResponse(it) }
    }
}