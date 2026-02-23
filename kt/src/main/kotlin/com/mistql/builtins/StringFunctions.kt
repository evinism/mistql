package com.mistql.builtins

import com.mistql.MistQLError
import com.mistql.ast.Expression
import com.mistql.runtime.ExecutionContext
import com.mistql.runtime.RuntimeValue

object StringFunctions {
    val all: Map<String, BuiltinFn> = mapOf(
        "split" to ::split,
        "stringjoin" to ::stringjoin,
        "replace" to ::replace,
        "match" to ::matchFn,
        "regex" to ::regex,
        "index" to ::index,
    )

    private fun split(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        validateArgs("split", args, 2, 2)
        val delimiter = eval(args[0], ctx)
        val str = assertStr(eval(args[1], ctx))

        val parts: List<String> = when (delimiter) {
            is RuntimeValue.Str -> {
                if (delimiter.value.isEmpty()) {
                    // Split by code points, not chars
                    val codePoints = str.codePoints().toArray()
                    codePoints.map { String(intArrayOf(it), 0, 1) }
                } else {
                    str.split(delimiter.value)
                }
            }
            is RuntimeValue.Regex -> {
                delimiter.compiled.split(str)
            }
            else -> throw MistQLError.Type("split: delimiter must be string or regex, got ${delimiter.typeName()}")
        }
        return RuntimeValue.Arr(parts.map { RuntimeValue.Str(it) })
    }

    private fun stringjoin(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        validateArgs("stringjoin", args, 2, 2)
        val delimiter = assertStr(eval(args[0], ctx))
        val arr = assertArr(eval(args[1], ctx))
        val strings = arr.map { item ->
            when (item) {
                is RuntimeValue.Str -> item.value
                is RuntimeValue.Func -> throw MistQLError.Type("Cannot cast function to string")
                is RuntimeValue.Regex -> throw MistQLError.Type("Cannot cast regex to string")
                else -> item.toStringSerialize()
            }
        }
        return RuntimeValue.Str(strings.joinToString(delimiter))
    }

    private fun replace(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        validateArgs("replace", args, 3, 3)
        val pattern = eval(args[0], ctx)
        val replacement = assertStr(eval(args[1], ctx))
        val target = assertStr(eval(args[2], ctx))

        return when (pattern) {
            is RuntimeValue.Str -> {
                // Replace first occurrence only
                val idx = target.indexOf(pattern.value)
                val result = if (idx >= 0) {
                    target.substring(0, idx) + replacement + target.substring(idx + pattern.value.length)
                } else {
                    target
                }
                RuntimeValue.Str(result)
            }
            is RuntimeValue.Regex -> {
                val result = if (pattern.flags.contains('g')) {
                    pattern.compiled.replace(target, Regex.escapeReplacement(replacement))
                } else {
                    pattern.compiled.replaceFirst(target, Regex.escapeReplacement(replacement))
                }
                RuntimeValue.Str(result)
            }
            else -> throw MistQLError.Type("replace: pattern must be string or regex, got ${pattern.typeName()}")
        }
    }

    private fun matchFn(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        validateArgs("match", args, 2, 2)
        val pattern = eval(args[0], ctx)
        val target = assertStr(eval(args[1], ctx))

        val matches = when (pattern) {
            is RuntimeValue.Str -> {
                val regex = Regex(pattern.value)
                regex.containsMatchIn(target)
            }
            is RuntimeValue.Regex -> {
                pattern.compiled.containsMatchIn(target)
            }
            else -> throw MistQLError.Type("match: pattern must be string or regex, got ${pattern.typeName()}")
        }
        return RuntimeValue.Bool(matches)
    }

    private fun regex(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        validateArgs("regex", args, 1, 2)
        val pattern = assertStr(eval(args[0], ctx))
        val flags = if (args.size == 2) assertStr(eval(args[1], ctx)) else ""
        return RuntimeValue.createRegex(pattern, flags)
    }

    private fun index(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        return when (args.size) {
            2 -> indexSingle(args, ctx)
            3 -> indexSlice(args, ctx)
            else -> throw MistQLError.Runtime("index takes 2 or 3 arguments, got ${args.size}")
        }
    }

    private fun indexSingle(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        val indexVal = eval(args[0], ctx)
        val operand = eval(args[1], ctx)

        return when (operand) {
            is RuntimeValue.Arr -> {
                val num = assertNum(indexVal)
                if (num != kotlin.math.floor(num)) throw MistQLError.Runtime("index: Non-integers cannot be used on arrays")
                val idx = num.toInt()
                val len = operand.items.size
                val actual = if (idx < 0) len + idx else idx
                if (actual in 0 until len) operand.items[actual] else RuntimeValue.Null
            }
            is RuntimeValue.Str -> {
                val num = assertNum(indexVal)
                if (num != kotlin.math.floor(num)) throw MistQLError.Runtime("index: Non-integers cannot be used on strings")
                val idx = num.toInt()
                val codePoints = operand.value.codePoints().toArray()
                val len = codePoints.size
                val actual = if (idx < 0) len + idx else idx
                if (actual in 0 until len) RuntimeValue.Str(String(intArrayOf(codePoints[actual]), 0, 1)) else RuntimeValue.Null
            }
            is RuntimeValue.Obj -> {
                val key = assertStr(indexVal)
                operand.entries[key] ?: RuntimeValue.Null
            }
            is RuntimeValue.Null -> {
                // Indexing null returns null, but only with number or string indices
                when (indexVal) {
                    is RuntimeValue.Num, is RuntimeValue.Str -> RuntimeValue.Null
                    else -> throw MistQLError.Type("Cannot index null with ${indexVal.typeName()}")
                }
            }
            else -> throw MistQLError.Type("Cannot index ${operand.typeName()}")
        }
    }

    private fun indexSlice(args: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        val startVal = eval(args[0], ctx)
        val endVal = eval(args[1], ctx)
        val operand = eval(args[2], ctx)

        return when (operand) {
            is RuntimeValue.Arr -> {
                val len = operand.items.size
                val start = resolveSliceIndex(startVal, 0, len)
                val end = resolveSliceIndex(endVal, len, len)
                val actualStart = start.coerceIn(0, len)
                val actualEnd = end.coerceIn(0, len)
                if (actualStart >= actualEnd) RuntimeValue.Arr(emptyList())
                else RuntimeValue.Arr(operand.items.subList(actualStart, actualEnd))
            }
            is RuntimeValue.Str -> {
                val codePoints = operand.value.codePoints().toArray()
                val len = codePoints.size
                val start = resolveSliceIndex(startVal, 0, len)
                val end = resolveSliceIndex(endVal, len, len)
                val actualStart = start.coerceIn(0, len)
                val actualEnd = end.coerceIn(0, len)
                if (actualStart >= actualEnd) RuntimeValue.Str("")
                else RuntimeValue.Str(String(codePoints, actualStart, actualEnd - actualStart))
            }
            is RuntimeValue.Null -> RuntimeValue.Null
            else -> throw MistQLError.Type("Cannot slice ${operand.typeName()}")
        }
    }

    private fun resolveSliceIndex(value: RuntimeValue, default: Int, len: Int): Int {
        return when (value) {
            is RuntimeValue.Null -> default
            is RuntimeValue.Num -> {
                val idx = value.value.toInt()
                if (idx < 0) len + idx else idx
            }
            else -> throw MistQLError.Type("Slice index must be number or null, got ${value.typeName()}")
        }
    }
}
