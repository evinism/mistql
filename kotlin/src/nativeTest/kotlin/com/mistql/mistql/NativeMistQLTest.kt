package com.mistql.mistql

import kotlin.test.Test
import kotlin.test.assertEquals

class NativeMistQLTest {
    @Test
    fun testEncodeToString() {
        assertQueryEquals("@", "hello", "hello")
    }

    private fun assertQueryEquals(input: String, data: String, expectedOutput: String) {
        assertEquals(expectedOutput, MistQLSessionFactory.createSession().query(input, data));
    }
}