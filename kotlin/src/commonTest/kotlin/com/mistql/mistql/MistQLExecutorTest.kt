package com.mistql.mistql

import kotlin.test.Test
import kotlin.test.assertEquals

import com.mistql.mistql.builtin.count

object V {
    fun nil() = Value.Null()
    fun arr(vararg entries: Value) = Value.Array(listOf(*entries))
    fun num(value: Double) = Value.Number(value)
    fun fn(imp: FunctionImplementation) = Value.Function(imp)
}

object E {
    fun ref(name: String) = ReferenceExpression(name)
    fun value(value: Value) = ValueExpression(value)
    fun ap(fn: Expression, args: List<Expression>) = ApplicationExpression(fn, args)
}

fun shortfn(imp: FunctionImplementation, vararg args: Expression) = E.ap(E.value(V.fn(imp)), listOf(*args))

class MistQLTest {
    @Test
    fun testEvalSimpleRef() {
        assertExecsTo(V.nil(), E.ref("@"), V.nil())
        assertExecsTo(V.arr(), E.ref("@"), V.arr())
    }

    @Test
    fun testSimpleApplication() {
        val expr = shortfn(count, E.ref("@"))
        val data = V.arr(V.nil(), V.nil(), V.nil())
        assertExecsTo(V.num(3.toDouble()), expr, data)
    }

    fun assertExecsTo(result: Value, ast: Expression, data: Value = V.nil()) {
        assertEquals(result, Executor.exec(ast, data))
    }
}