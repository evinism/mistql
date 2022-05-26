package com.mistql.mistql

interface MistQL {
    fun query(src: String, data: String): String
}

expect object MistQLSessionFactory {
    fun createSession(): MistQL
}