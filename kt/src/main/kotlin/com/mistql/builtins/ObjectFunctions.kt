package com.mistql.builtins

import com.mistql.ast.Expression
import com.mistql.runtime.ExecutionContext
import com.mistql.runtime.RuntimeValue

object ObjectFunctions {
    val all: Map<String, BuiltinFn> = mapOf(
        "keys" to ::keys,
        "values" to ::values,
        "entries" to ::entries,
        "fromentries" to ::fromentries,
        "mapkeys" to ::mapkeys,
        "mapvalues" to ::mapvalues,
        "filterkeys" to ::filterkeys,
        "filtervalues" to ::filtervalues,
    )

    private fun keys(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        validateArgs("keys", args, 1, 1)
        val obj = assertObj(eval(args[0], ctx))
        val sortedKeys = obj.keys.sorted()
        return RuntimeValue.Arr(sortedKeys.map { RuntimeValue.Str(it) })
    }

    private fun values(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        validateArgs("values", args, 1, 1)
        val v = eval(args[0], ctx)
        if (v !is RuntimeValue.Obj) return RuntimeValue.Arr(emptyList())
        val sortedKeys = v.entries.keys.sorted()
        return RuntimeValue.Arr(sortedKeys.map { v.entries[it]!! })
    }

    private fun entries(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        validateArgs("entries", args, 1, 1)
        val obj = assertObj(eval(args[0], ctx))
        val sortedKeys = obj.keys.sorted()
        return RuntimeValue.Arr(sortedKeys.map { key ->
            RuntimeValue.Arr(listOf(RuntimeValue.Str(key), obj[key]!!))
        })
    }

    private fun fromentries(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        validateArgs("fromentries", args, 1, 1)
        val arr = assertArr(eval(args[0], ctx))
        val result = LinkedHashMap<String, RuntimeValue>()
        for (entry in arr) {
            val entryArr = assertArr(entry)
            val key = if (entryArr.isNotEmpty()) entryArr[0].toStringDisplay() else "null"
            val value = if (entryArr.size > 1) entryArr[1] else RuntimeValue.Null
            result[key] = value
        }
        return RuntimeValue.Obj(result)
    }

    private fun mapkeys(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        validateArgs("mapkeys", args, 2, 2)
        val transform = args[0]
        val obj = assertObj(eval(args[1], ctx))
        val result = LinkedHashMap<String, RuntimeValue>()
        for ((key, value) in obj) {
            ctx.pushContext(RuntimeValue.Str(key))
            val newKey = evalContextualized(transform, ctx)
            ctx.popContext()
            val keyStr = when (newKey) {
                is RuntimeValue.Obj, is RuntimeValue.Arr -> newKey.toStringSerialize()
                else -> newKey.toStringDisplay()
            }
            result[keyStr] = value
        }
        return RuntimeValue.Obj(result)
    }

    private fun mapvalues(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        validateArgs("mapvalues", args, 2, 2)
        val transform = args[0]
        val obj = assertObj(eval(args[1], ctx))
        val result = LinkedHashMap<String, RuntimeValue>()
        for ((key, value) in obj) {
            ctx.pushContext(value)
            val newValue = evalContextualized(transform, ctx)
            ctx.popContext()
            result[key] = newValue
        }
        return RuntimeValue.Obj(result)
    }

    private fun filterkeys(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        validateArgs("filterkeys", args, 2, 2)
        val condition = args[0]
        val obj = assertObj(eval(args[1], ctx))
        val result = LinkedHashMap<String, RuntimeValue>()
        for ((key, value) in obj) {
            ctx.pushContext(RuntimeValue.Str(key))
            val condResult = evalContextualized(condition, ctx)
            ctx.popContext()
            if (condResult.truthy()) {
                result[key] = value
            }
        }
        return RuntimeValue.Obj(result)
    }

    private fun filtervalues(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        validateArgs("filtervalues", args, 2, 2)
        val condition = args[0]
        val obj = assertObj(eval(args[1], ctx))
        val result = LinkedHashMap<String, RuntimeValue>()
        for ((key, value) in obj) {
            ctx.pushContext(value)
            val condResult = evalContextualized(condition, ctx)
            ctx.popContext()
            if (condResult.truthy()) {
                result[key] = value
            }
        }
        return RuntimeValue.Obj(result)
    }
}
