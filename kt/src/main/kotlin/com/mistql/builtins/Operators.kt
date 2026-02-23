package com.mistql.builtins

import com.mistql.MistQLError
import com.mistql.runtime.RuntimeValue

object Operators {
    val all: Map<String, BuiltinFn> = mapOf(
        "!/unary" to ::notUnary,
        "-/unary" to ::negUnary,
        "||" to ::or,
        "&&" to ::and,
        "==" to ::eq,
        "!=" to ::neq,
        "=~" to ::regexMatch,
        ">" to ::gt,
        "<" to ::lt,
        ">=" to ::gte,
        "<=" to ::lte,
        "+" to ::add,
        "-" to ::sub,
        "*" to ::mul,
        "/" to ::div,
        "%" to ::mod,
        "." to ::dot,
    )

    private fun notUnary(args: List<com.mistql.ast.Expression>, ctx: com.mistql.runtime.ExecutionContext): RuntimeValue {
        validateArgs("!/unary", args, 1, 1)
        return RuntimeValue.Bool(!eval(args[0], ctx).truthy())
    }

    private fun negUnary(args: List<com.mistql.ast.Expression>, ctx: com.mistql.runtime.ExecutionContext): RuntimeValue {
        validateArgs("-/unary", args, 1, 1)
        val v = eval(args[0], ctx)
        return RuntimeValue.Num(-assertNum(v))
    }

    private fun or(args: List<com.mistql.ast.Expression>, ctx: com.mistql.runtime.ExecutionContext): RuntimeValue {
        validateArgs("||", args, 2, 2)
        val left = eval(args[0], ctx)
        return if (left.truthy()) left else eval(args[1], ctx)
    }

    private fun and(args: List<com.mistql.ast.Expression>, ctx: com.mistql.runtime.ExecutionContext): RuntimeValue {
        validateArgs("&&", args, 2, 2)
        val left = eval(args[0], ctx)
        return if (left.truthy()) eval(args[1], ctx) else left
    }

    private fun eq(args: List<com.mistql.ast.Expression>, ctx: com.mistql.runtime.ExecutionContext): RuntimeValue {
        validateArgs("==", args, 2, 2)
        return RuntimeValue.Bool(eval(args[0], ctx).equal(eval(args[1], ctx)))
    }

    private fun neq(args: List<com.mistql.ast.Expression>, ctx: com.mistql.runtime.ExecutionContext): RuntimeValue {
        validateArgs("!=", args, 2, 2)
        return RuntimeValue.Bool(!eval(args[0], ctx).equal(eval(args[1], ctx)))
    }

    private fun regexMatch(args: List<com.mistql.ast.Expression>, ctx: com.mistql.runtime.ExecutionContext): RuntimeValue {
        validateArgs("=~", args, 2, 2)
        val target = eval(args[0], ctx)
        val pattern = eval(args[1], ctx)
        val targetStr = assertStr(target)
        return when (pattern) {
            is RuntimeValue.Regex -> RuntimeValue.Bool(pattern.compiled.containsMatchIn(targetStr))
            is RuntimeValue.Str -> {
                val regex = Regex(pattern.value)
                RuntimeValue.Bool(regex.containsMatchIn(targetStr))
            }
            else -> throw MistQLError.Type("=~ requires regex or string pattern, got ${pattern.typeName()}")
        }
    }

    private fun gt(args: List<com.mistql.ast.Expression>, ctx: com.mistql.runtime.ExecutionContext): RuntimeValue {
        validateArgs(">", args, 2, 2)
        return RuntimeValue.Bool(eval(args[0], ctx).compare(eval(args[1], ctx)) > 0)
    }

    private fun lt(args: List<com.mistql.ast.Expression>, ctx: com.mistql.runtime.ExecutionContext): RuntimeValue {
        validateArgs("<", args, 2, 2)
        return RuntimeValue.Bool(eval(args[0], ctx).compare(eval(args[1], ctx)) < 0)
    }

    private fun gte(args: List<com.mistql.ast.Expression>, ctx: com.mistql.runtime.ExecutionContext): RuntimeValue {
        validateArgs(">=", args, 2, 2)
        return RuntimeValue.Bool(eval(args[0], ctx).compare(eval(args[1], ctx)) >= 0)
    }

    private fun lte(args: List<com.mistql.ast.Expression>, ctx: com.mistql.runtime.ExecutionContext): RuntimeValue {
        validateArgs("<=", args, 2, 2)
        return RuntimeValue.Bool(eval(args[0], ctx).compare(eval(args[1], ctx)) <= 0)
    }

    private fun add(args: List<com.mistql.ast.Expression>, ctx: com.mistql.runtime.ExecutionContext): RuntimeValue {
        validateArgs("+", args, 2, 2)
        val left = eval(args[0], ctx)
        val right = eval(args[1], ctx)
        return when {
            left is RuntimeValue.Num && right is RuntimeValue.Num -> RuntimeValue.Num(left.value + right.value)
            left is RuntimeValue.Str && right is RuntimeValue.Str -> RuntimeValue.Str(left.value + right.value)
            left is RuntimeValue.Arr && right is RuntimeValue.Arr -> RuntimeValue.Arr(left.items + right.items)
            else -> throw MistQLError.Type("Cannot add ${left.typeName()} and ${right.typeName()}")
        }
    }

    private fun sub(args: List<com.mistql.ast.Expression>, ctx: com.mistql.runtime.ExecutionContext): RuntimeValue {
        validateArgs("-", args, 2, 2)
        return RuntimeValue.Num(assertNum(eval(args[0], ctx)) - assertNum(eval(args[1], ctx)))
    }

    private fun mul(args: List<com.mistql.ast.Expression>, ctx: com.mistql.runtime.ExecutionContext): RuntimeValue {
        validateArgs("*", args, 2, 2)
        return RuntimeValue.Num(assertNum(eval(args[0], ctx)) * assertNum(eval(args[1], ctx)))
    }

    private fun div(args: List<com.mistql.ast.Expression>, ctx: com.mistql.runtime.ExecutionContext): RuntimeValue {
        validateArgs("/", args, 2, 2)
        val left = assertNum(eval(args[0], ctx))
        val right = assertNum(eval(args[1], ctx))
        if (right == 0.0) throw MistQLError.Runtime("Division by zero")
        return RuntimeValue.Num(left / right)
    }

    private fun mod(args: List<com.mistql.ast.Expression>, ctx: com.mistql.runtime.ExecutionContext): RuntimeValue {
        validateArgs("%", args, 2, 2)
        val left = assertNum(eval(args[0], ctx))
        val right = assertNum(eval(args[1], ctx))
        if (right == 0.0) throw MistQLError.Runtime("Modulo by zero")
        return RuntimeValue.Num(left % right)
    }

    private fun dot(args: List<com.mistql.ast.Expression>, ctx: com.mistql.runtime.ExecutionContext): RuntimeValue {
        validateArgs(".", args, 2, 2)
        val obj = eval(args[0], ctx)
        val fieldExpr = args[1]
        val fieldName = when (fieldExpr) {
            is com.mistql.ast.Expression.Ref -> fieldExpr.name
            else -> assertStr(eval(fieldExpr, ctx))
        }
        return when (obj) {
            is RuntimeValue.Obj -> obj.entries[fieldName] ?: RuntimeValue.Null
            is RuntimeValue.Null -> RuntimeValue.Null
            else -> throw MistQLError.Type("Cannot access property '$fieldName' on type ${obj.typeName()}")
        }
    }
}
