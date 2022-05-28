package com.mistql.mistql

enum class BuiltinFunction {
    APPLY,
    COUNT,
    ENTRIES,
    FILTER,
    FILTERKEYS,
    FILTERVALUES,
    FIND,
    FLATTEN,
    FLOAT,
    FROMENTRIES,
    GROUPBY,
    IF_FN,
    INDEX,
    STRINGJOIN,
    KEYS,
    LOG,
    MATCH,
    MAP,
    MAPKEYS,
    MAPVALUES,
    REDUCE,
    REGEX,
    REPLACE,
    REVERSE,
    SEQUENCE,
    SORT,
    SORTBY,
    SPLIT,
    STRING,
    SUM,
    SUMMARIZE,
    VALUES,
    WITHINDICES,
    OP_BANG_UNARY,
    OP_MINUS_UNARY,
    OP_DOT,
    OP_PLUS,
    OP_MINUS,
    OP_MULTIPLY,
    OP_DIVIDE,
    OP_MODULO,
    OP_OR,
    OP_AND,
    OP_EQUAL,
    OP_NOTEQUAL,
    OP_GT,
    OP_LT,
    OP_GTE,
    OP_LTE,
    OP_MATCH,
}

sealed class Value {
    class Null() : Value() {
        override fun equals(other: Any?): kotlin.Boolean = true
    }
    data class String(val value: String) : Value()
    data class Number(val value: kotlin.String) : Value()
    data class Boolean(val value: kotlin.Boolean) : Value()
    data class Object(val entries: Map<String, Value> = mapOf()) : Value()
    data class Array(val entries: List<Value> = emptyList()) : Value()
    data class Function(val id: BuiltinFunction) : Value()
    data class Regex(val pattern: kotlin.String, val flags: kotlin.String) : Value()
}