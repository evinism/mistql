package com.mistql.mistql

actual object MistQLSessionFactory {
    actual fun createSession(): MistQL = NativeMistQLSession
}

object NativeMistQLSession : MistQL {
    override fun query(str: String, data: String): String = data
}