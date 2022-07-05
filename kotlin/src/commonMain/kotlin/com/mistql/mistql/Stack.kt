package com.mistql.mistql

data class StackFrame (
    val entries: Map<String, out Value>
)

data class Stack (val frames: List<StackFrame> = emptyList()) {
    fun getReference(name: String): Value? {
        for (item in frames.asReversed()) {
            val entry = item.entries.get(name)
            if (entry != null) {
                return entry;
            }
        }
        return null;
    }

    fun withContextValue(value: Value): Stack {
        val newStackFrameEntries: MutableMap<String, Value> = mutableMapOf()
        if (value is Value.Object) {
            newStackFrameEntries.putAll(value.entries)
        }
        newStackFrameEntries.put("@", value)
        val newStackFrame = StackFrame(newStackFrameEntries)
        return Stack(frames + newStackFrame)
    }
}