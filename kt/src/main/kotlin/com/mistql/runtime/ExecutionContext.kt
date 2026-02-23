package com.mistql.runtime

import com.mistql.MistQLError
import com.mistql.builtins.Builtins

class ExecutionContext(val rootData: RuntimeValue) {
    private val stack: MutableList<MutableMap<String, RuntimeValue>> = mutableListOf()

    init {
        // Frame 0: $ frame containing builtins + root data
        val dollarFrame = mutableMapOf<String, RuntimeValue>()
        dollarFrame["@"] = rootData
        for (name in Builtins.allNames()) {
            dollarFrame[name] = RuntimeValue.Func(name)
        }

        val rootFrame = mutableMapOf<String, RuntimeValue>()
        rootFrame["$"] = RuntimeValue.Obj(LinkedHashMap(dollarFrame))
        stack.add(rootFrame)

        // Frame 1: Data context
        pushContext(rootData)
    }

    fun pushContext(value: RuntimeValue) {
        val newFrame = mutableMapOf<String, RuntimeValue>()
        newFrame["@"] = value

        // If value is an object, populate keys as variables
        if (value is RuntimeValue.Obj) {
            for ((key, v) in value.entries) {
                if (isValidIdentifier(key)) {
                    newFrame[key] = v
                }
            }
        }
        stack.add(newFrame)
    }

    fun popContext() {
        if (stack.size <= 2) {
            throw MistQLError.Runtime("Cannot pop initial stack frames")
        }
        stack.removeAt(stack.lastIndex)
    }

    fun findVariable(name: String, internal: Boolean): RuntimeValue {
        if (internal) {
            // For internal references, check builtins directly
            if (Builtins.isBuiltin(name)) {
                return RuntimeValue.Func(name)
            }
            throw MistQLError.Runtime("Could not find referenced variable $name")
        }

        // Search from top to bottom
        for (i in stack.indices.reversed()) {
            val frame = stack[i]
            val value = frame[name]
            if (value != null) {
                return value
            }
        }

        // Check builtins as fallback
        if (Builtins.isBuiltin(name)) {
            return RuntimeValue.Func(name)
        }

        throw MistQLError.Runtime("Could not find referenced variable $name")
    }

    companion object {
        private val identifierRegex = Regex("^[a-zA-Z_][a-zA-Z_0-9]*$")

        fun isValidIdentifier(s: String): Boolean {
            return identifierRegex.matches(s)
        }
    }
}
