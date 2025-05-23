/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.logging

import com.google.common.base.Throwables
import java.time.ZoneId
import java.time.ZonedDateTime
import java.util.logging.Formatter
import java.util.logging.LogRecord
import ninja.blacknet.TERMINAL_WIDTH

private val UTC = ZoneId.of("UTC")

/**
 * A [Formatter] that does not disclose a locale and a time zone.
 */
class Formatter : Formatter() {
    override fun format(record: LogRecord): String {
        val instant = record.getInstant().atZone(UTC)
        return buildString(TERMINAL_WIDTH * 2 + 4) {
            zeroed(4, instant.getYear())
            append('-')
            zeroed(2, instant.getMonthValue())
            append('-')
            zeroed(2, instant.getDayOfMonth())
            append(' ')
            zeroed(2, instant.getHour())
            append(':')
            zeroed(2, instant.getMinute())
            append(':')
            zeroed(2, instant.getSecond())
            append('.')
            zeroed(3, instant.getMilli())
            append(' ')
            //append(record.getLoggerName())
            //append(' ')
            append(record.getSourceClassName())
            append(' ')
            append(record.getSourceMethodName())
            append(System.lineSeparator())
            append(record.getLevel().getName())
            append(": ")
            append(record.getMessage())
            append(System.lineSeparator())
            record.getThrown()?.let { e ->
                append(Throwables.getStackTraceAsString(e))
                append(System.lineSeparator())
            }
        }
    }

    override fun formatMessage(record: LogRecord): String {
        // this method is supposed to localize, but that compromises anonymity
        return format(record)
    }

    private fun StringBuilder.zeroed(length: Int, int: Int) {
        val string = int.toString()
        val padding = length - string.length
        for (i in 0 until padding)
            append('0')
        append(string)
    }

    private fun ZonedDateTime.getMilli(): Int {
        return getNano() / 1000000
    }
}
