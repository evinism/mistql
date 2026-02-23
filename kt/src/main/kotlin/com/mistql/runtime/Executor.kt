package com.mistql.runtime

import com.mistql.MistQLError
import com.mistql.ast.Expression
import com.mistql.builtins.Builtins

object Executor {
    fun execute(expr: Expression, ctx: ExecutionContext): RuntimeValue {
        return when (expr) {
            is Expression.Value -> expr.value
            is Expression.Ref -> ctx.findVariable(expr.name, expr.internal)
            is Expression.FnCall -> executeFnCall(expr, ctx)
            is Expression.Pipe -> executePipeline(expr.stages, ctx)
            is Expression.ArrayLiteral -> RuntimeValue.Arr(expr.items.map { execute(it, ctx) })
            is Expression.ObjectLiteral -> {
                val map = LinkedHashMap<String, RuntimeValue>()
                for ((k, v) in expr.entries) {
                    map[k] = execute(v, ctx)
                }
                RuntimeValue.Obj(map)
            }
            is Expression.Paren -> execute(expr.expression, ctx)
        }
    }

    private fun executeFnCall(expr: Expression.FnCall, ctx: ExecutionContext): RuntimeValue {
        val funcValue = execute(expr.function, ctx)
        if (funcValue !is RuntimeValue.Func) {
            throw MistQLError.Runtime("Attempted to call a variable of type \"${funcValue.typeName()}\". Only functions are callable")
        }
        val builtin = Builtins.get(funcValue.name)
            ?: throw MistQLError.Runtime("Unknown function: ${funcValue.name}")
        return builtin(expr.arguments, ctx)
    }

    private fun executePipeline(stages: List<Expression>, ctx: ExecutionContext): RuntimeValue {
        if (stages.isEmpty()) {
            throw MistQLError.Runtime("Empty pipeline")
        }

        var data = execute(stages[0], ctx)

        for (i in 1 until stages.size) {
            val stage = stages[i] as Expression.FnCall
            ctx.pushContext(data)

            // Append @ reference as the last argument (like JS does)
            val atRef = Expression.Ref("@")
            val newCall = Expression.FnCall(stage.function, stage.arguments + atRef)
            val result = execute(newCall, ctx)

            ctx.popContext()
            data = result
        }

        return data
    }

    /**
     * Execute an expression in a contextualized way.
     * If the expression is a bare function reference, call it with @ as argument.
     * Otherwise, evaluate the expression directly.
     */
    fun executeContextualized(expr: Expression, ctx: ExecutionContext): RuntimeValue {
        if (expr is Expression.Ref && !expr.internal) {
            val value = try { ctx.findVariable(expr.name, false) } catch (_: Exception) { null }
            if (value is RuntimeValue.Func) {
                val call = Expression.FnCall(expr, listOf(Expression.Ref("@")))
                return execute(call, ctx)
            }
        }
        return execute(expr, ctx)
    }
}
