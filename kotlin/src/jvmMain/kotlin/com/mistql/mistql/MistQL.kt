package com.mistql.mistql


actual object MistQLSessionFactory {
    actual fun createSession(): MistQL = JVMMistQLSession
}

object JVMMistQLSession : MistQL {
    override fun query(src: String, data: String): String = data
}