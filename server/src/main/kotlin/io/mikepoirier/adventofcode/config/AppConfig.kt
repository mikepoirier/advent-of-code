package io.mikepoirier.adventofcode.config

import org.slf4j.LoggerFactory
import org.springframework.boot.context.event.ApplicationReadyEvent
import org.springframework.boot.context.properties.ConfigurationProperties
import org.springframework.boot.context.properties.EnableConfigurationProperties
import org.springframework.context.annotation.Configuration
import org.springframework.context.event.EventListener

@Configuration
@EnableConfigurationProperties(ApplicationProperties::class)
class ApplicationConfig(val applicationProperties: ApplicationProperties) {

    private val log = LoggerFactory.getLogger(ApplicationConfig::class.java)

    @EventListener
    fun onStartupComplete(applicationReadyEvent: ApplicationReadyEvent) {
        log.info("Application Properties: $applicationProperties")
    }
}

@ConfigurationProperties(prefix = "adventofcode")
class ApplicationProperties(var host: String = "http://localhost", var session: String = "session") {

    override fun toString(): String {
        return "ApplicationProperties(host=$host, session=$session)"
    }

    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as ApplicationProperties

        if (host != other.host) return false
        if (session != other.session) return false

        return true
    }

    override fun hashCode(): Int {
        var result = host.hashCode()
        result = 31 * result + (session.hashCode())
        return result
    }
}