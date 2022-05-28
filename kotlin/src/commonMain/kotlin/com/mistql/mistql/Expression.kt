package com.mistql.mistql

interface Expression {
    abstract fun exec(stack: Stack): Value
}

data class ReferenceExpression(val name: String) : Expression {
    override fun exec(stack: Stack): Value = stack.getReference(name) ?: Value.Null()
}

data class ApplicationExpression(val fn: Expression, val args: List<Expression>) : Expression {
    override fun exec(stack: Stack): Value {
        val lhs = fn.exec(stack)
        if (lhs is Value.Function) {
            return lhs.implementation.apply(args, stack)
        }
        throw Error("Can't call a non-function!")
    }
}

data class ValueExpression(val value: Value) : Expression {
    override fun exec(stack: Stack): Value = value
}

data class ArrayLiteralExpression(val values: List<Expression>) : Expression {
    override fun exec(stack: Stack): Value = Value.Array(values.map { return it.exec(stack) })
}

data class ObjectLiteralExpression(val entries: Map<String, Expression>) : Expression {
    override fun exec(stack: Stack): Value = Value.Object(entries.mapValues {
        return it.value.exec(stack)
    })
}

data class PipeExpression(val segments: List<Expression>) : Expression {
    override fun exec(stack: Stack): Value = throw NotImplementedError("Pipes not implemented")
}