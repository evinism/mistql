package com.mistql

import com.mistql.parser.Parser
import com.mistql.runtime.ExecutionContext
import com.mistql.runtime.Executor
import com.mistql.runtime.RuntimeValue
import kotlinx.serialization.json.JsonElement

sealed class MistQLError(message: String) : Exception(message) {
    class Parse(message: String) : MistQLError("Parse error: $message")
    class Runtime(message: String) : MistQLError("Runtime error: $message")
    class Type(message: String) : MistQLError("Type error: $message")
}

object MistQL {
    fun query(queryStr: String, data: Any?): Any? {
        val runtimeData = RuntimeValue.fromAny(data)
        val expr = Parser.parse(queryStr)
        val ctx = ExecutionContext(runtimeData)
        val result = Executor.execute(expr, ctx)
        return result.toAny()
    }

    fun queryRuntime(queryStr: String, data: RuntimeValue): RuntimeValue {
        val expr = Parser.parse(queryStr)
        val ctx = ExecutionContext(data)
        return Executor.execute(expr, ctx)
    }

    fun queryJson(queryStr: String, data: JsonElement): JsonElement {
        val runtimeData = RuntimeValue.fromJson(data)
        val expr = Parser.parse(queryStr)
        val ctx = ExecutionContext(runtimeData)
        val result = Executor.execute(expr, ctx)
        return result.toJson()
    }
}
