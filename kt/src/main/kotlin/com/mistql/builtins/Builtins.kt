package com.mistql.builtins

import com.mistql.MistQLError
import com.mistql.ast.Expression
import com.mistql.runtime.ExecutionContext
import com.mistql.runtime.Executor
import com.mistql.runtime.RuntimeValue

typealias BuiltinFn = (args: List<Expression>, ctx: ExecutionContext) -> RuntimeValue

object Builtins {
    private val registry: Map<String, BuiltinFn> = buildMap {
        putAll(Operators.all)
        putAll(ArrayFunctions.all)
        putAll(ObjectFunctions.all)
        putAll(StringFunctions.all)
        putAll(MathFunctions.all)
        putAll(UtilityFunctions.all)
    }

    fun get(name: String): BuiltinFn? = registry[name]
    fun allNames(): Set<String> = registry.keys
    fun isBuiltin(name: String): Boolean = name in registry
}

// Helpers
fun validateArgs(name: String, args: List<Expression>, min: Int, max: Int? = null) {
    if (args.size < min) {
        throw MistQLError.Runtime("$name takes at least $min arguments, got ${args.size}")
    }
    if (max != null && args.size > max) {
        throw MistQLError.Runtime("$name takes at most $max arguments, got ${args.size}")
    }
}

fun eval(expr: Expression, ctx: ExecutionContext): RuntimeValue = Executor.execute(expr, ctx)
fun evalContextualized(expr: Expression, ctx: ExecutionContext): RuntimeValue = Executor.executeContextualized(expr, ctx)

fun assertNum(v: RuntimeValue): Double = when (v) {
    is RuntimeValue.Num -> v.value
    else -> throw MistQLError.Type("Expected number, got ${v.typeName()}")
}

fun assertArr(v: RuntimeValue): List<RuntimeValue> = when (v) {
    is RuntimeValue.Arr -> v.items
    else -> throw MistQLError.Type("Expected array, got ${v.typeName()}")
}

fun assertStr(v: RuntimeValue): String = when (v) {
    is RuntimeValue.Str -> v.value
    else -> throw MistQLError.Type("Expected string, got ${v.typeName()}")
}

fun assertObj(v: RuntimeValue): LinkedHashMap<String, RuntimeValue> = when (v) {
    is RuntimeValue.Obj -> v.entries
    else -> throw MistQLError.Type("Expected object, got ${v.typeName()}")
}
