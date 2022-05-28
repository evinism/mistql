package com.mistql.mistql

data class StackFrame (
    val entries: Map<String, out Value>
)

data class Stack (val frames: List<StackFrame>) {
    fun getReference(name: String): Value? {
        for (item in frames.asReversed()) {
            val entry = item.entries.get(name)
            if (entry != null) {
                return entry;
            }
        }
        return null;
    }
}