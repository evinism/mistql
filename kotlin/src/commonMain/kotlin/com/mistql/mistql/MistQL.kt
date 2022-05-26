package com.mistql.mistql

import kotlinx.serialization.*
import kotlinx.serialization.json.*


interface MistQLSession {
    fun query(src: String, data: String): String
}

expect object MistQLSessionFactory {
    fun createSession(): MistQLSession
}

@Serializable
data class Project(val name: String, val language: String)


object CommonMistQLSession : MistQLSession {
    override fun query(query: String, data: String): String {
        if (query == "@") {
            return data;
        }
        return Json.encodeToString(Project("blah", "blah"));
    }
}