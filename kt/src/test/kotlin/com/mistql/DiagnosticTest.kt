package com.mistql

import org.junit.jupiter.api.Test
import kotlin.test.assertEquals

class DiagnosticTest {
    @Test
    fun testBasicFunctionCalls() {
        // string 1 should call string(1) and return "1"
        assertEquals("1", MistQL.query("string 1", null))
    }

    @Test
    fun testCount() {
        assertEquals(3L, MistQL.query("count @", listOf(1, 2, 3)))
    }

    @Test
    fun testArithmetic() {
        assertEquals(3L, MistQL.query("1 + 2", null))
    }

    @Test
    fun testPipe() {
        assertEquals(3L, MistQL.query("[1, 2, 3] | count", null))
    }

    @Test
    fun testFilter() {
        assertEquals(listOf(2L, 3L), MistQL.query("[1, 2, 3] | filter @ > 1", null))
    }

    @Test
    fun testDotAccess() {
        val data = mapOf("name" to "John")
        assertEquals("John", MistQL.query("name", data))
    }

    @Test
    fun testDollarAccess() {
        val data = mapOf("filter" to "hp")
        // $.filter should access the builtin filter function, not the data "filter" key
        // This requires $ handling
    }

    @Test
    fun testRegexLiteral() {
        assertEquals(true, MistQL.query("!!(regex \"hi\")", null))
    }
}
