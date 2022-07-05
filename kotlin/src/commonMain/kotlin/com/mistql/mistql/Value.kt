package com.mistql.mistql

sealed class Value {
    class Null() : Value() {
        override fun equals(other: Any?): kotlin.Boolean = true
    }
    data class String(val value: kotlin.String) : Value()
    data class Number(val value: kotlin.Double) : Value()
    data class Boolean(val value: kotlin.Boolean) : Value()
    data class Object(val entries: Map<kotlin.String, Value> = mapOf()) : Value()
    data class Array(val entries: List<Value> = emptyList()) : Value()
    data class Function(val implementation: FunctionImplementation) : Value()
    data class Regex(val pattern: kotlin.String, val flags: kotlin.String) : Value()
}