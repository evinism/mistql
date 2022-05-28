package com.mistql.mistql

import kotlin.test.Test
import kotlin.test.assertEquals

class MistQLExecutorTest {
    @Test
    fun testEncodeToString() {
        assertQueryEquals("@", "{}", "{}")
    }

    private fun assertQueryEquals(input: String, data: String, expectedOutput: String) {
        assertEquals(expectedOutput, CommonMistQLSession.query(input, data));
    }
}