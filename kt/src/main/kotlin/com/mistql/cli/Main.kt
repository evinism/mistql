package com.mistql.cli

import com.mistql.MistQL
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonElement

fun main(args: Array<String>) {
    if (args.isEmpty()) {
        System.err.println("Usage: mq <query> [file]")
        System.exit(1)
    }

    val queryStr = args[0]
    val json = Json { ignoreUnknownKeys = true }

    val input = if (args.size > 1) {
        java.io.File(args[1]).readText()
    } else {
        System.`in`.bufferedReader().readText()
    }

    val data: JsonElement = json.parseToJsonElement(input)
    val result = MistQL.queryJson(queryStr, data)

    val output = json.encodeToString(JsonElement.serializer(), result)
    println(output)
}
