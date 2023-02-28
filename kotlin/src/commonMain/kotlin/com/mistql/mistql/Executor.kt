package com.mistql.mistql

object Executor {
    fun getDefaultStack(context: Value): Stack {
        return Stack().withContextValue(context)
    }

    fun exec(expr: Expression, context: Value): Value {
        return expr.exec(getDefaultStack(context))
    }
}