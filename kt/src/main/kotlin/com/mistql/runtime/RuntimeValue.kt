package com.mistql.runtime

import com.mistql.MistQLError
import kotlinx.serialization.json.*

sealed class RuntimeValue {
    object Null : RuntimeValue() {
        override fun toString(): String = "null"
    }
    data class Bool(val value: Boolean) : RuntimeValue()
    data class Num(val value: Double) : RuntimeValue()
    data class Str(val value: String) : RuntimeValue()
    data class Arr(val items: List<RuntimeValue>) : RuntimeValue()
    data class Obj(val entries: LinkedHashMap<String, RuntimeValue>) : RuntimeValue()
    data class Func(val name: String) : RuntimeValue()
    data class Regex(val pattern: String, val flags: String, val compiled: kotlin.text.Regex) : RuntimeValue() {
        override fun equals(other: Any?): Boolean {
            if (this === other) return true
            if (other !is Regex) return false
            return pattern == other.pattern && flags == other.flags
        }
        override fun hashCode(): Int = pattern.hashCode() * 31 + flags.hashCode()
    }

    fun truthy(): Boolean = when (this) {
        is Null -> false
        is Bool -> value
        is Num -> value != 0.0
        is Str -> value.isNotEmpty()
        is Arr -> items.isNotEmpty()
        is Obj -> entries.isNotEmpty()
        is Func -> true
        is Regex -> true
    }

    fun typeName(): String = when (this) {
        is Null -> "null"
        is Bool -> "boolean"
        is Num -> "number"
        is Str -> "string"
        is Arr -> "array"
        is Obj -> "object"
        is Func -> "function"
        is Regex -> "regex"
    }

    fun comparable(): Boolean = this is Bool || this is Num || this is Str

    fun compare(other: RuntimeValue): Int {
        if (this.typeName() != other.typeName()) {
            throw MistQLError.Runtime("Cannot compare MistQL values of different types")
        }
        if (!this.comparable()) {
            throw MistQLError.Runtime("Cannot compare MistQL values of type ${this.typeName()}")
        }
        return when {
            this is Num && other is Num -> this.value.compareTo(other.value)
            this is Str && other is Str -> this.value.compareTo(other.value)
            this is Bool && other is Bool -> this.value.compareTo(other.value)
            else -> throw MistQLError.Runtime("Cannot compare MistQL values of type ${this.typeName()}")
        }
    }

    fun equal(other: RuntimeValue): Boolean {
        if (this.typeName() != other.typeName()) return false
        return when {
            this is Null && other is Null -> true
            this is Bool && other is Bool -> this.value == other.value
            this is Num && other is Num -> this.value == other.value
            this is Str && other is Str -> this.value == other.value
            this is Arr && other is Arr -> {
                if (this.items.size != other.items.size) false
                else this.items.zip(other.items).all { (a, b) -> a.equal(b) }
            }
            this is Obj && other is Obj -> {
                if (this.entries.size != other.entries.size) false
                else {
                    val aKeys = this.entries.keys.sorted()
                    val bKeys = other.entries.keys.sorted()
                    if (aKeys != bKeys) false
                    else aKeys.all { key -> this.entries[key]!!.equal(other.entries[key]!!) }
                }
            }
            this is Func && other is Func -> this.name == other.name
            this is Regex && other is Regex -> this.pattern == other.pattern && this.flags == other.flags
            else -> false
        }
    }

    fun toStringDisplay(): String = when (this) {
        is Null -> "null"
        is Bool -> value.toString()
        is Num -> formatNumber(value)
        is Str -> value
        is Arr -> "[${items.joinToString(",") { it.toStringSerialize() }}]"
        is Obj -> "{${entries.entries.joinToString(",") { "\"${it.key}\":${it.value.toStringSerialize()}" }}}"
        is Func -> "[function]"
        is Regex -> "[regex]"
    }

    fun toStringSerialize(): String = when (this) {
        is Null -> "null"
        is Bool -> value.toString()
        is Num -> formatNumber(value)
        is Str -> "\"${escapeJsonString(value)}\""
        is Arr -> "[${items.joinToString(",") { it.toStringSerialize() }}]"
        is Obj -> "{${entries.entries.joinToString(",") { "\"${escapeJsonString(it.key)}\":${it.value.toStringSerialize()}" }}}"
        is Func -> "[function]"
        is Regex -> "[regex]"
    }

    fun toJson(): JsonElement = when (this) {
        is Null -> JsonNull
        is Bool -> JsonPrimitive(value)
        is Num -> JsonPrimitive(value)
        is Str -> JsonPrimitive(value)
        is Arr -> JsonArray(items.map { it.toJson() })
        is Obj -> JsonObject(entries.mapValues { it.value.toJson() })
        is Func -> throw MistQLError.Runtime("Cannot convert function to JSON")
        is Regex -> throw MistQLError.Runtime("Cannot convert regex to JSON")
    }

    fun toAny(): Any? = when (this) {
        is Null -> null
        is Bool -> value
        is Num -> if (value == kotlin.math.floor(value) && !value.isInfinite() && kotlin.math.abs(value) < Long.MAX_VALUE.toDouble()) value.toLong() else value
        is Str -> value
        is Arr -> items.map { it.toAny() }
        is Obj -> entries.mapValues { it.value.toAny() }
        is Func -> throw MistQLError.Runtime("Cannot return function as output")
        is Regex -> throw MistQLError.Runtime("Cannot return regex as output")
    }

    companion object {
        fun fromJson(element: JsonElement): RuntimeValue = when (element) {
            is JsonNull -> Null
            is JsonPrimitive -> when {
                element.isString -> Str(element.content)
                element.content == "true" -> Bool(true)
                element.content == "false" -> Bool(false)
                else -> Num(element.double)
            }
            is JsonArray -> Arr(element.map { fromJson(it) })
            is JsonObject -> Obj(LinkedHashMap<String, RuntimeValue>().also { map ->
                element.forEach { (k, v) -> map[k] = fromJson(v) }
            })
        }

        fun fromAny(value: Any?): RuntimeValue = when (value) {
            null -> Null
            is Boolean -> Bool(value)
            is Number -> {
                val d = value.toDouble()
                if (d.isNaN() || d.isInfinite()) Null
                else Num(d)
            }
            is String -> Str(value)
            is List<*> -> Arr(value.map { fromAny(it) })
            is Map<*, *> -> Obj(LinkedHashMap<String, RuntimeValue>().also { map ->
                value.forEach { (k, v) -> map[k.toString()] = fromAny(v) }
            })
            is JsonElement -> fromJson(value)
            else -> throw MistQLError.Type("Cannot convert ${value::class.simpleName} to RuntimeValue")
        }

        fun formatNumber(n: Double): String {
            if (n.isNaN()) return "NaN"
            if (n.isInfinite()) return if (n > 0) "Infinity" else "-Infinity"

            // Use Double.toString() for the canonical representation, then reformat
            // to match JavaScript's Number.toString() output
            val javaStr = n.toString()

            // Handle scientific notation from Java (e.g., "3.141592653589793E50")
            val eIdx = javaStr.indexOf('E')
            if (eIdx >= 0) {
                val mantissa = javaStr.substring(0, eIdx)
                val exp = javaStr.substring(eIdx + 1).toInt()

                val abs = kotlin.math.abs(n)
                if (abs >= 1e21 || (abs != 0.0 && abs < 1e-6)) {
                    // JS uses scientific notation
                    val cleanMantissa = mantissa.trimEnd('0').trimEnd('.')
                    val expStr = if (exp >= 0) "e+$exp" else "e$exp"
                    return "$cleanMantissa$expStr"
                }

                // Otherwise expand to fixed notation
                return formatFixed(n, abs)
            }

            // No scientific notation from Java
            val abs = kotlin.math.abs(n)
            if (abs != 0.0 && abs < 1e-6) {
                // JS would use scientific notation
                return formatToScientific(n)
            }
            if (abs >= 1e21) {
                return formatToScientific(n)
            }

            // Regular number - Java's toString is usually fine
            // But remove trailing ".0" for integers
            if (n == kotlin.math.floor(n) && abs < 1e15 && javaStr.endsWith(".0")) {
                return javaStr.dropLast(2)
            }
            return javaStr
        }

        private fun formatFixed(n: Double, abs: Double): String {
            // For numbers in the range [1e-6, 1e21), format as fixed-point
            return if (n == kotlin.math.floor(n) && abs < 1e15) {
                "%.0f".format(n)
            } else {
                val formatted = "%.20f".format(n)
                formatted.trimEnd('0').trimEnd('.')
            }
        }

        private fun formatToScientific(n: Double): String {
            // Convert a double to JS-style scientific notation
            val javaStr = n.toString()
            val eIdx = javaStr.indexOf('E')
            if (eIdx >= 0) {
                val mantissa = javaStr.substring(0, eIdx).trimEnd('0').trimEnd('.')
                val exp = javaStr.substring(eIdx + 1).toInt()
                val expStr = if (exp >= 0) "e+$exp" else "e$exp"
                return "$mantissa$expStr"
            }
            // Shouldn't happen, but fallback
            return javaStr
        }

        private fun escapeJsonString(s: String): String {
            val sb = StringBuilder()
            for (c in s) {
                when (c) {
                    '"' -> sb.append("\\\"")
                    '\\' -> sb.append("\\\\")
                    '\b' -> sb.append("\\b")
                    '\u000C' -> sb.append("\\f")
                    '\n' -> sb.append("\\n")
                    '\r' -> sb.append("\\r")
                    '\t' -> sb.append("\\t")
                    else -> {
                        if (c.code < 0x20) {
                            sb.append("\\u%04x".format(c.code))
                        } else {
                            sb.append(c)
                        }
                    }
                }
            }
            return sb.toString()
        }

        fun createRegex(pattern: String, flags: String): Regex {
            val options = mutableSetOf<RegexOption>()
            for (flag in flags) {
                when (flag) {
                    'i' -> options.add(RegexOption.IGNORE_CASE)
                    'm' -> options.add(RegexOption.MULTILINE)
                    's' -> options.add(RegexOption.DOT_MATCHES_ALL)
                    'g' -> {} // handled at call site
                    'x' -> options.add(RegexOption.COMMENTS)
                    else -> throw MistQLError.Runtime("Invalid regex flag: $flag")
                }
            }
            val compiled = kotlin.text.Regex(pattern, options)
            return Regex(pattern, flags, compiled)
        }
    }
}
