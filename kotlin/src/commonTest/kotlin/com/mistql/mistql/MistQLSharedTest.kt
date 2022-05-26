package com.mistql.mistql

import kotlin.test.Test
import kotlin.test.assertEquals

import kotlinx.serialization.*
import kotlinx.serialization.json.*
import java.util.Optional

@Serializable
data class TestCaseAssertion(val query: String, val data: JsonElement);

@Serializable
data class TestCase(val it: String, val assertions: List<TestCaseAssertion>);

@Serializable
data class TestSubDomain(val describe: String, val cases: List<TestCase>);

@Serializable
data class TestDomain(val describe: String, val cases: List<TestSubDomain>);

@Serializable
data class TestSuite(val data: List<TestDomain>);

val json = Json {
    ignoreUnknownKeys = true
}

class MistQLSharedTest {
    @Test
    fun sharedTestSuite() {
        val testSuiteContent = this::class.java.classLoader.getResource("shared/testdata.json").readText();
        val res = json.parseToJsonElement(testSuiteContent);
        val item = json.decodeFromString<TestSuite>(testSuiteContent);
        assertEquals(res, CommonMistQLSession.query("@", res));
    }
}