package com.mistql.ast

import com.mistql.runtime.RuntimeValue

sealed class Expression {
    data class Value(val value: RuntimeValue) : Expression()
    data class Ref(val name: String, val internal: Boolean = false) : Expression()
    data class FnCall(val function: Expression, val arguments: List<Expression>, val isExplicitCall: Boolean = false) : Expression()
    data class Pipe(val stages: List<Expression>) : Expression()
    data class ArrayLiteral(val items: List<Expression>) : Expression()
    data class ObjectLiteral(val entries: List<Pair<String, Expression>>) : Expression()
    data class Paren(val expression: Expression) : Expression()
}
