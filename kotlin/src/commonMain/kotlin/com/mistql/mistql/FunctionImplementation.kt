package com.mistql.mistql

abstract class FunctionImplementation {
    abstract fun apply(args: List<Expression>, stack: Stack): Value
}
