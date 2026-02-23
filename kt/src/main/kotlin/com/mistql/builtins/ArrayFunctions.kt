package com.mistql.builtins

import com.mistql.MistQLError
import com.mistql.ast.Expression
import com.mistql.runtime.ExecutionContext
import com.mistql.runtime.RuntimeValue

object ArrayFunctions {
    val all: Map<String, BuiltinFn> = mapOf(
        "count" to ::count,
        "filter" to ::filter,
        "map" to ::map,
        "find" to ::find,
        "reverse" to ::reverse,
        "flatten" to ::flatten,
        "sum" to ::sum,
        "sort" to ::sort,
        "sortby" to ::sortby,
        "reduce" to ::reduce,
        "groupby" to ::groupby,
        "withindices" to ::withindices,
        "sequence" to ::sequence,
    )

    private fun count(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        validateArgs("count", args, 1, 1)
        val arr = assertArr(eval(args[0], ctx))
        return RuntimeValue.Num(arr.size.toDouble())
    }

    private fun filter(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        validateArgs("filter", args, 2, 2)
        val condition = args[0]
        val arr = assertArr(eval(args[1], ctx))

        val result = mutableListOf<RuntimeValue>()
        for (item in arr) {
            ctx.pushContext(item)
            val condResult = evalContextualized(condition, ctx)
            ctx.popContext()
            if (condResult.truthy()) {
                result.add(item)
            }
        }
        return RuntimeValue.Arr(result)
    }

    private fun map(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        validateArgs("map", args, 2, 2)
        val transform = args[0]
        val arr = assertArr(eval(args[1], ctx))

        val result = mutableListOf<RuntimeValue>()
        for (item in arr) {
            ctx.pushContext(item)
            val transformed = evalContextualized(transform, ctx)
            ctx.popContext()
            result.add(transformed)
        }
        return RuntimeValue.Arr(result)
    }

    private fun find(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        validateArgs("find", args, 2, 2)
        val condition = args[0]
        val arr = assertArr(eval(args[1], ctx))

        for (item in arr) {
            ctx.pushContext(item)
            val condResult = evalContextualized(condition, ctx)
            ctx.popContext()
            if (condResult.truthy()) {
                return item
            }
        }
        return RuntimeValue.Null
    }

    private fun reverse(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        validateArgs("reverse", args, 1, 1)
        return RuntimeValue.Arr(assertArr(eval(args[0], ctx)).reversed())
    }

    private fun flatten(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        validateArgs("flatten", args, 1, 1)
        val arr = assertArr(eval(args[0], ctx))
        val result = mutableListOf<RuntimeValue>()
        for (item in arr) {
            when (item) {
                is RuntimeValue.Arr -> result.addAll(item.items)
                else -> throw MistQLError.Type("flatten: all elements must be arrays, got ${item.typeName()}")
            }
        }
        return RuntimeValue.Arr(result)
    }

    private fun sum(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        validateArgs("sum", args, 1, 1)
        val arr = assertArr(eval(args[0], ctx))
        var total = 0.0
        for (item in arr) {
            total += assertNum(item)
        }
        return RuntimeValue.Num(total)
    }

    private fun sort(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        validateArgs("sort", args, 1, 1)
        val arr = assertArr(eval(args[0], ctx))
        if (arr.isEmpty()) return RuntimeValue.Arr(arr)

        val firstType = arr[0].typeName()
        for (item in arr) {
            if (item.typeName() != firstType) {
                throw MistQLError.Type("sort: cannot sort non-homogeneous arrays (found $firstType and ${item.typeName()})")
            }
            if (!item.comparable()) {
                throw MistQLError.Runtime("sort: Cannot sort non-comparable values")
            }
        }

        return RuntimeValue.Arr(arr.sortedWith { a, b -> a.compare(b) })
    }

    private fun sortby(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        validateArgs("sortby", args, 2, 2)
        val keyExpr = args[0]
        val arr = assertArr(eval(args[1], ctx))

        val withKeys = arr.map { item ->
            ctx.pushContext(item)
            val key = evalContextualized(keyExpr, ctx)
            ctx.popContext()
            Pair(key, item)
        }

        // Validate all keys are comparable and same type
        if (withKeys.isNotEmpty()) {
            val firstType = withKeys[0].first.typeName()
            for ((key, _) in withKeys) {
                if (!key.comparable()) {
                    throw MistQLError.Runtime("Cannot compare MistQL values of type ${key.typeName()}")
                }
                if (key.typeName() != firstType) {
                    throw MistQLError.Runtime("Cannot compare MistQL values of different types")
                }
            }
        }

        return RuntimeValue.Arr(
            withKeys.sortedWith { a, b -> a.first.compare(b.first) }.map { it.second }
        )
    }

    private fun reduce(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        validateArgs("reduce", args, 3, 3)
        val reducerExpr = args[0]
        var accumulator = eval(args[1], ctx)
        val arr = assertArr(eval(args[2], ctx))

        for (item in arr) {
            val accCur = RuntimeValue.Arr(listOf(accumulator, item))
            ctx.pushContext(accCur)
            accumulator = eval(reducerExpr, ctx)
            ctx.popContext()
        }
        return accumulator
    }

    private fun groupby(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        validateArgs("groupby", args, 2, 2)
        val keyExpr = args[0]
        val arr = assertArr(eval(args[1], ctx))

        val groups = LinkedHashMap<String, MutableList<RuntimeValue>>()
        for (item in arr) {
            ctx.pushContext(item)
            val key = evalContextualized(keyExpr, ctx)
            ctx.popContext()
            val keyStr = key.toStringDisplay()
            groups.getOrPut(keyStr) { mutableListOf() }.add(item)
        }

        val result = LinkedHashMap<String, RuntimeValue>()
        for ((key, values) in groups) {
            result[key] = RuntimeValue.Arr(values)
        }
        return RuntimeValue.Obj(result)
    }

    private fun withindices(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        validateArgs("withindices", args, 1, 1)
        val arr = assertArr(eval(args[0], ctx))
        return RuntimeValue.Arr(arr.mapIndexed { index, item ->
            RuntimeValue.Arr(listOf(RuntimeValue.Num(index.toDouble()), item))
        })
    }

    private fun sequence(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        if (args.size < 2) throw MistQLError.Runtime("sequence takes at least 2 arguments")
        val predicates = args.subList(0, args.size - 1)
        val arr = assertArr(eval(args.last(), ctx))

        // Create bitmasks for each predicate
        val bitmasks = predicates.map { predicate ->
            arr.map { item ->
                ctx.pushContext(item)
                val result = eval(predicate, ctx)
                ctx.popContext()
                result.truthy()
            }
        }

        val indices = sequenceHelper(bitmasks, 0)
        return RuntimeValue.Arr(indices.map { idxList ->
            RuntimeValue.Arr(idxList.map { idx -> arr[idx] })
        })
    }

    private fun sequenceHelper(bitmasks: List<List<Boolean>>, start: Int): List<List<Int>> {
        if (bitmasks.isEmpty()) return emptyList()
        val first = bitmasks[0]
        val result = mutableListOf<List<Int>>()

        for (idx in start until first.size) {
            if (first[idx]) {
                if (bitmasks.size == 1) {
                    result.add(listOf(idx))
                } else {
                    val subResults = sequenceHelper(bitmasks.subList(1, bitmasks.size), idx + 1)
                    for (sub in subResults) {
                        result.add(listOf(idx) + sub)
                    }
                }
            }
        }
        return result
    }
}
