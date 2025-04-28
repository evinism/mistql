package com.mistql.mistql

actual object MistQLSessionFactory {
    actual fun createSession(): MistQLSession = CommonMistQLSession
}
