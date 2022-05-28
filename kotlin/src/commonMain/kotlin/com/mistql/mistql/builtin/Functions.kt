package com.mistql.mistql.builtin

import com.mistql.mistql.FunctionImplementation
import com.mistql.mistql.Expression
import com.mistql.mistql.Value
import com.mistql.mistql.Stack

object count : FunctionImplementation() {
    override fun apply(args: List<Expression>, stack: Stack): Value {
        val res = args[0].exec(stack)
        if (res is Value.Array) {
            return Value.Number(res.entries.size.toDouble())
        } else {
            throw Error("Wrong function type!")
        }
    }
}
