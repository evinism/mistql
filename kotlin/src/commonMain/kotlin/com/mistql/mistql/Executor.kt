package com.mistql.mistql

object Executor {
    fun getDefaultStack(context: Value): Stack {
        val frames: List<StackFrame> = listOf(
            StackFrame(
                mapOf(
                    "@" to context
                )
            )
        )
        return Stack(frames)
    }

    fun exec(expr: Expression, context: Value): Value {
        return expr.exec(getDefaultStack(context))
    }
}