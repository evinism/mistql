package com.mistql.mistql

interface Expression {
    abstract fun exec(stack: Stack): Value
}

data class ReferenceExpression(val name: String) : Expression {
    override fun exec(stack: Stack) = stack.getReference(name) ?: Value.Null()
}

data class ApplicationExpression(val fn: Expression, val args: List<Expression>) : Expression {
    override fun exec(stack: Stack) = Value.Null()
}

data class ValueExpression(val value: Value) : Expression {
    override fun exec(stack: Stack) = Value.Null()
}

data class ArrayLiteralExpression(val values: List<Expression>) : Expression {
    override fun exec(stack: Stack) = Value.Null()
}

data class ObjectLiteralExpression(val entries: Pair<String, Expression>) : Expression {
    override fun exec(stack: Stack) = Value.Null()
}

data class PipeExpression(val segments: List<Expression>) : Expression {
    override fun exec(stack: Stack) = Value.Null()
}