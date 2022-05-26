package com.mistql.mistql

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith


import kotlinx.serialization.*
import kotlinx.serialization.json.*
import java.lang.Exception
import java.util.Optional

@Serializable
data class TestCaseAssertion(
    val query: String,
    val data: JsonElement,
    val expected: JsonElement = JsonNull,
    val throws: Boolean = false
);

@Serializable
data class TestCase(
    val it: String,
    val skip: List<String> = emptyList(),
    val assertions: List<TestCaseAssertion>);

@Serializable
data class TestSubDomain(val describe: String, val cases: List<TestCase>);

@Serializable
data class TestDomain(val describe: String, val cases: List<TestSubDomain>);

@Serializable
data class TestSuite(val data: List<TestDomain>);

class MistQLSharedTest {
    @Test
    fun sharedTestSuite() {
        val testSuiteContent = this::class.java.classLoader.getResource("shared/testdata.json").readText();
        val res = Json.parseToJsonElement(testSuiteContent);
        val testSuite = Json.decodeFromString<TestSuite>(testSuiteContent);
        testSuite.data.forEach {
            it.cases.forEach {
                it.cases.forEach {
                    it.assertions.forEach {
                        if (it.throws) {
                            assertFailsWith<Exception>(
                                block = {
                                    CommonMistQLSession.query(it.query, it.data)
                                }
                            )
                        } else {
                            assertEquals(it.expected, CommonMistQLSession.query(it.query, it.data));
                        }
                    }
                }
            }
        }
    }
}