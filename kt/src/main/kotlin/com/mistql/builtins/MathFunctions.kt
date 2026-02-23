package com.mistql.builtins

import com.mistql.MistQLError
import com.mistql.ast.Expression
import com.mistql.runtime.ExecutionContext
import com.mistql.runtime.RuntimeValue
import kotlin.math.sqrt

object MathFunctions {
    val all: Map<String, BuiltinFn> = mapOf(
        "range" to ::range,
        "summarize" to ::summarize,
    )

    private fun range(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        validateArgs("range", args, 1, 3)

        val start: Long
        val stop: Long
        val step: Long

        when (args.size) {
            1 -> {
                start = 0
                stop = assertInteger(eval(args[0], ctx), "stop")
                step = 1
            }
            2 -> {
                start = assertInteger(eval(args[0], ctx), "start")
                stop = assertInteger(eval(args[1], ctx), "stop")
                step = 1
            }
            else -> {
                start = assertInteger(eval(args[0], ctx), "start")
                stop = assertInteger(eval(args[1], ctx), "stop")
                step = assertInteger(eval(args[2], ctx), "step")
            }
        }

        if (step == 0L) throw MistQLError.Runtime("range: step must not be 0")

        val result = mutableListOf<RuntimeValue>()
        if (step > 0) {
            var current = start
            while (current < stop) {
                result.add(RuntimeValue.Num(current.toDouble()))
                current += step
            }
        } else {
            var current = start
            while (current > stop) {
                result.add(RuntimeValue.Num(current.toDouble()))
                current += step
            }
        }
        return RuntimeValue.Arr(result)
    }

    private fun summarize(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        validateArgs("summarize", args, 1, 1)
        val arr = assertArr(eval(args[0], ctx))
        if (arr.isEmpty()) throw MistQLError.Runtime("summarize: cannot summarize empty array")

        val numbers = arr.map { assertNum(it) }
        if (numbers.size < 2) throw MistQLError.Runtime("summarize: requires at least 2 numbers")

        val max = numbers.max()
        val min = numbers.min()
        val mean = numbers.sum() / numbers.size
        val sorted = numbers.sorted()
        val median = if (sorted.size % 2 == 0) {
            (sorted[sorted.size / 2 - 1] + sorted[sorted.size / 2]) / 2.0
        } else {
            sorted[sorted.size / 2]
        }
        val variance = numbers.sumOf { (it - mean) * (it - mean) } / (numbers.size - 1)
        val stddev = sqrt(variance)

        val result = LinkedHashMap<String, RuntimeValue>()
        result["max"] = RuntimeValue.Num(max)
        result["min"] = RuntimeValue.Num(min)
        result["mean"] = RuntimeValue.Num(mean)
        result["median"] = RuntimeValue.Num(median)
        result["variance"] = RuntimeValue.Num(variance)
        result["stddev"] = RuntimeValue.Num(stddev)
        return RuntimeValue.Obj(result)
    }

    private fun assertInteger(v: RuntimeValue, name: String): Long {
        val n = assertNum(v)
        if (n != kotlin.math.floor(n)) {
            throw MistQLError.Type("range: $name must be an integer, got $n")
        }
        return n.toLong()
    }
}
