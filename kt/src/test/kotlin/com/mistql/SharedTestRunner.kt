package com.mistql

import com.mistql.runtime.RuntimeValue
import kotlinx.serialization.json.*
import org.junit.jupiter.api.Assumptions
import org.junit.jupiter.api.DynamicTest
import org.junit.jupiter.api.TestFactory
import java.io.File
import kotlin.test.assertEquals
import kotlin.test.assertTrue
import kotlin.test.assertFailsWith

class SharedTestRunner {
    companion object {
        private const val SELF_LANG_ID = "kt"
    }

    @TestFactory
    fun sharedTests(): List<DynamicTest> {
        val testdataFile = File("shared/testdata.json")
        if (!testdataFile.exists()) {
            return listOf(DynamicTest.dynamicTest("shared/testdata.json not found") {
                Assumptions.assumeTrue(false, "shared/testdata.json not found")
            })
        }

        val json = Json { ignoreUnknownKeys = true }
        val testdata = json.parseToJsonElement(testdataFile.readText()).jsonObject
        val dataBlocks = testdata["data"]!!.jsonArray

        val tests = mutableListOf<DynamicTest>()

        for (block in dataBlocks) {
            val blockDescribe = block.jsonObject["describe"]!!.jsonPrimitive.content
            val blockCases = block.jsonObject["cases"]!!.jsonArray

            for (innerBlock in blockCases) {
                val innerDescribe = innerBlock.jsonObject["describe"]!!.jsonPrimitive.content
                val innerCases = innerBlock.jsonObject["cases"]!!.jsonArray

                for (testCase in innerCases) {
                    val itDescription = testCase.jsonObject["it"]!!.jsonPrimitive.content
                    val skip = testCase.jsonObject["skip"]?.jsonArray?.map { it.jsonPrimitive.content }
                    val assertions = testCase.jsonObject["assertions"]!!.jsonArray

                    val testName = "$blockDescribe > $innerDescribe > $itDescription"

                    if (skip?.contains(SELF_LANG_ID) == true) {
                        tests.add(DynamicTest.dynamicTest("[SKIP] $testName") {
                            Assumptions.assumeTrue(false, "Skipped for $SELF_LANG_ID")
                        })
                        continue
                    }

                    for ((i, assertion) in assertions.withIndex()) {
                        val query = assertion.jsonObject["query"]!!.jsonPrimitive.content
                        val data = assertion.jsonObject["data"]!!
                        val expected = assertion.jsonObject["expected"]
                        val expectedSet = assertion.jsonObject["expectedSet"]?.jsonArray
                        val throws = assertion.jsonObject["throws"]?.jsonPrimitive?.booleanOrNull

                        tests.add(DynamicTest.dynamicTest("$testName #${i + 1}: $query") {
                            if (throws == true) {
                                assertFailsWith<Exception> {
                                    MistQL.queryJson(query, data)
                                }
                            } else if (expectedSet != null) {
                                val result = MistQL.queryJson(query, data)
                                val matches = expectedSet.any { jsonElementsEqual(result, it) }
                                assertTrue(matches, "Result $result did not match any of $expectedSet")
                            } else if (expected != null) {
                                val result = MistQL.queryJson(query, data)
                                assertTrue(
                                    jsonElementsEqual(result, expected),
                                    "Expected $expected but got $result for query: $query"
                                )
                            }
                        })
                    }
                }
            }
        }

        return tests
    }

    /**
     * Deep equality for JsonElement that handles number comparison correctly.
     * JSON numbers like 6 and 6.0 should be considered equal.
     */
    private fun jsonElementsEqual(a: JsonElement, b: JsonElement): Boolean {
        return when {
            a is JsonNull && b is JsonNull -> true
            a is JsonPrimitive && b is JsonPrimitive -> {
                if (a.isString && b.isString) {
                    a.content == b.content
                } else if (!a.isString && !b.isString) {
                    // Both are non-string primitives (numbers or booleans)
                    // Try numeric comparison first
                    val aDouble = a.doubleOrNull
                    val bDouble = b.doubleOrNull
                    if (aDouble != null && bDouble != null) {
                        aDouble == bDouble
                    } else {
                        a.content == b.content
                    }
                } else {
                    false
                }
            }
            a is JsonArray && b is JsonArray -> {
                a.size == b.size && a.zip(b).all { (x, y) -> jsonElementsEqual(x, y) }
            }
            a is JsonObject && b is JsonObject -> {
                a.keys == b.keys && a.keys.all { key -> jsonElementsEqual(a[key]!!, b[key]!!) }
            }
            else -> false
        }
    }
}
