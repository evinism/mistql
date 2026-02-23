package com.mistql.builtins

import com.mistql.MistQLError
import com.mistql.ast.Expression
import com.mistql.runtime.ExecutionContext
import com.mistql.runtime.RuntimeValue

object UtilityFunctions {
    val all: Map<String, BuiltinFn> = mapOf(
        "log" to ::log,
        "if" to ::ifFn,
        "apply" to ::apply,
        "string" to ::stringFn,
        "float" to ::floatFn,
        "type" to ::typeFn,
        "not" to ::notFn,
        "empty" to ::emptyFn,
        "error" to ::errorFn,
    )

    private fun log(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        validateArgs("log", args, 1, 1)
        val value = eval(args[0], ctx)
        println("MistQL: ${value.toStringSerialize()}")
        return value
    }

    private fun ifFn(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        validateArgs("if", args, 3, 3)
        val condition = eval(args[0], ctx)
        return if (condition.truthy()) eval(args[1], ctx) else eval(args[2], ctx)
    }

    private fun apply(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        validateArgs("apply", args, 2, 2)
        val expression = args[0]
        val value = eval(args[1], ctx)
        ctx.pushContext(value)
        val result = evalContextualized(expression, ctx)
        ctx.popContext()
        return result
    }

    private fun stringFn(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        validateArgs("string", args, 1, 1)
        val value = eval(args[0], ctx)
        return when (value) {
            is RuntimeValue.Str -> value
            is RuntimeValue.Func -> throw MistQLError.Type("Cannot cast type function to string")
            is RuntimeValue.Regex -> throw MistQLError.Type("Cannot cast type regex to string")
            else -> {
                assertNoFunctionsOrRegexes(value)
                RuntimeValue.Str(value.toStringSerialize())
            }
        }
    }

    private fun assertNoFunctionsOrRegexes(value: RuntimeValue) {
        when (value) {
            is RuntimeValue.Func -> throw MistQLError.Type("Cannot cast type function to string")
            is RuntimeValue.Regex -> throw MistQLError.Type("Cannot cast type regex to string")
            is RuntimeValue.Arr -> value.items.forEach { assertNoFunctionsOrRegexes(it) }
            is RuntimeValue.Obj -> value.entries.values.forEach { assertNoFunctionsOrRegexes(it) }
            else -> {}
        }
    }

    private fun floatFn(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        validateArgs("float", args, 1, 1)
        val value = eval(args[0], ctx)
        val num = when (value) {
            is RuntimeValue.Num -> value.value
            is RuntimeValue.Str -> {
                val trimmed = value.value.trim()
                trimmed.toDoubleOrNull()
                    ?: throw MistQLError.Type("Cannot convert string to float: ${value.value}")
            }
            is RuntimeValue.Bool -> if (value.value) 1.0 else 0.0
            is RuntimeValue.Null -> 0.0
            else -> throw MistQLError.Type("Cannot convert ${value.typeName()} to float")
        }
        return RuntimeValue.Num(num)
    }

    private fun typeFn(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        validateArgs("type", args, 1, 1)
        return RuntimeValue.Str(eval(args[0], ctx).typeName())
    }

    private fun notFn(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        validateArgs("not", args, 1, 1)
        return RuntimeValue.Bool(!eval(args[0], ctx).truthy())
    }

    private fun emptyFn(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        validateArgs("empty", args, 1, 1)
        val value = eval(args[0], ctx)
        return RuntimeValue.Bool(!value.truthy())
    }

    private fun errorFn(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        validateArgs("error", args, 1, 1)
        val msg = assertStr(eval(args[0], ctx))
        throw MistQLError.Runtime(msg)
    }
}
