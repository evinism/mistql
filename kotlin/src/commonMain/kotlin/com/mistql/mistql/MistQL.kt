package com.mistql.mistql

import kotlinx.serialization.*
import kotlinx.serialization.json.*


interface MistQLSession {
    fun query(src: String, data: String): String
    fun query(src: String, data: JsonElement): JsonElement
}

expect object MistQLSessionFactory {
    fun createSession(): MistQLSession
}


object CommonMistQLSession : MistQLSession {
    override fun query(query: String, data: JsonElement): JsonElement {
        if (query == "@") {
            return data;
        }
        return  Json.parseToJsonElement("null");
    }

    override fun query(query: String, data: String): String {
        return  Json.encodeToString(query(query, Json.parseToJsonElement(data)))
    }
}