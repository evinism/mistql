package com.mistql.mistql

import kotlin.test.Test
import kotlin.test.assertEquals


object V {
    fun nil() = Value.Null()
    fun arr(entries: List<Value> = emptyList()) = Value.Array(entries)
}

object E {
    fun ref(name: String) = ReferenceExpression(name)
    fun value(value: Value) = ValueExpression(value)
}

class MistQLTest {
    @Test
    fun testEvalSimpleRef() {
        assertExecsTo(V.nil(), E.ref("@"), V.nil())
        assertExecsTo(V.arr(), E.ref("@"), V.arr())
    }

    fun assertExecsTo(result: Value, ast: Expression, data: Value = V.nil()) {
        assertEquals(result, Executor.exec(ast, data))
    }
}