package com.mistql.parser

import com.mistql.MistQLError
import com.mistql.ast.Expression
import com.mistql.runtime.RuntimeValue

/**
 * Parser for MistQL that follows the JS implementation's approach:
 * 1. Lex into tokens (value, ref, special)
 * 2. Collect items and joiners (operators) in a flat pass
 * 3. Resolve precedence by collapsing operators from highest to lowest
 * 4. Handle "amalgamating" operators (space for function application, | for pipeline)
 */
class Parser(private val input: String) {
    private var pos: Int = 0

    companion object {
        fun parse(input: String): Expression {
            val parser = Parser(input)
            val tokens = parser.lex()
            return parseTokens(tokens, input)
        }

        // Operator precedence: highest first
        private val SIMPLE_BINARY_OPERATORS = listOf(
            listOf("."),
            listOf("*", "/", "%"),
            listOf("+", "-"),
            listOf("<", ">", "<=", ">="),
            listOf("==", "!=", "=~"),
            listOf("&&"),
            listOf("||"),
        )

        private val AMALGAMATING_OPERATORS = listOf(" ", "|")

        private val ALL_BINARY_OPS: Set<String> = buildSet {
            for (level in SIMPLE_BINARY_OPERATORS) addAll(level)
            addAll(AMALGAMATING_OPERATORS)
        }

        private val UNARY_OPS = setOf("-", "!")

        private val SPECIALS: Set<String> = buildSet {
            addAll(ALL_BINARY_OPS)
            addAll(UNARY_OPS)
            addAll(listOf("(", ")", "[", "]", "{", "}", ":", ","))
        }

        private val BUILTIN_VALUES = mapOf("true" to true, "false" to false, "null" to null)
        private val NUM_REGEX = Regex("^(0|([1-9][0-9]*))(\\.[0-9]*)?([eE][+-]?[0-9]+)?")

        // Operators that vacuum whitespace on right side
        private val VACUUM_RIGHT = setOf("(", "{", "[")
        // Operators that vacuum whitespace on left side
        private val VACUUM_LEFT = setOf(")", "}", "]")
        // Operators that vacuum whitespace on both sides
        private val VACUUM_BOTH: Set<String> = buildSet {
            add(".")
            add(":")
            add("|")
            add(",")
            for (level in SIMPLE_BINARY_OPERATORS) addAll(level)
            addAll(AMALGAMATING_OPERATORS.filter { it != " " })
        }

        private fun vacuumsWhitespace(token: String, direction: String): Boolean {
            if (token in VACUUM_BOTH) return true
            return when (direction) {
                "l" -> token in VACUUM_LEFT
                "r" -> token in VACUUM_RIGHT
                else -> false
            }
        }

        // ────────── Token-based parsing ──────────

        private fun parseTokens(tokens: List<Token>, rawQuery: String): Expression {
            val (result, offset) = consumeExpression(tokens, 0, rawQuery)
            if (offset != tokens.size) {
                throw MistQLError.Parse("Unexpected token ${tokens[offset].value}, expected EOF")
            }
            return result
        }

        private fun consumeExpression(tokens: List<Token>, startOffset: Int, rawQuery: String): Pair<Expression, Int> {
            val items = mutableListOf<Expression>()
            val joiners = mutableListOf<String>()
            var offset = startOffset

            while (offset < tokens.size) {
                val next = tokens[offset]

                // Determine if we should parse a unary operator
                val unaryProcessors = mutableListOf<(Expression) -> Expression>()
                if (next.type == TokenType.SPECIAL && next.value.toString() in UNARY_OPS && joiners.size + 1 != items.size) {
                    // Collect consecutive unary operators
                    var i = offset
                    while (i < tokens.size && tokens[i].type == TokenType.SPECIAL && tokens[i].value.toString() in UNARY_OPS) {
                        val op = tokens[i].value.toString()
                        unaryProcessors.add { operand ->
                            Expression.FnCall(Expression.Ref("${op}/unary", internal = true), listOf(operand))
                        }
                        i++
                    }
                    offset = i
                    if (offset >= tokens.size) {
                        throw MistQLError.Parse("Expected expression after unary operator")
                    }
                }

                val token = if (offset < tokens.size) tokens[offset] else break

                when {
                    // Parenthetical
                    token.type == TokenType.SPECIAL && token.value == "(" -> {
                        guardItem(items, joiners, token, rawQuery)
                        val (result, newOffset) = consumeParenthetical(tokens, offset, rawQuery)
                        items.add(applyUnaries(result, unaryProcessors))
                        offset = newOffset
                    }

                    // Dot access - postfix on previous item
                    token.type == TokenType.SPECIAL && token.value == "." -> {
                        if (items.isEmpty()) throw MistQLError.Parse("Unexpected '.'")
                        val (result, newOffset) = consumeDotAccess(items.removeLast(), tokens, offset, rawQuery)
                        items.add(result)
                        offset = newOffset
                    }

                    // [ - either array literal or indexer
                    token.type == TokenType.SPECIAL && token.value == "[" -> {
                        if (joiners.size == items.size) {
                            // Item position -> array literal
                            val (result, newOffset) = consumeArray(tokens, offset, rawQuery)
                            items.add(applyUnaries(result, unaryProcessors))
                            offset = newOffset
                        } else {
                            // Postfix position -> indexer on previous item
                            val (indexApp, newOffset) = consumeIndexer(tokens, offset, rawQuery)
                            val prev = items.removeLast()
                            // indexApp is FnCall(Ref("index"), indexArgs) - append prev as last arg
                            val indexCall = indexApp as Expression.FnCall
                            items.add(Expression.FnCall(indexCall.function, indexCall.arguments + prev))
                            offset = newOffset
                        }
                    }

                    // Object literal
                    token.type == TokenType.SPECIAL && token.value == "{" -> {
                        guardItem(items, joiners, token, rawQuery)
                        val (result, newOffset) = consumeStruct(tokens, offset, rawQuery)
                        items.add(applyUnaries(result, unaryProcessors))
                        offset = newOffset
                    }

                    // Value literal
                    token.type == TokenType.VALUE -> {
                        guardItem(items, joiners, token, rawQuery)
                        val expr = valueLiteral(token.value)
                        items.add(applyUnaries(expr, unaryProcessors))
                        offset++
                    }

                    // Reference
                    token.type == TokenType.REF -> {
                        guardItem(items, joiners, token, rawQuery)
                        val expr = Expression.Ref(token.value.toString())
                        items.add(applyUnaries(expr, unaryProcessors))
                        offset++
                    }

                    // Binary operator
                    token.type == TokenType.SPECIAL && token.value.toString() in ALL_BINARY_OPS -> {
                        if (unaryProcessors.isNotEmpty()) {
                            throw MistQLError.Parse("Unexpected token ${token.value}")
                        }
                        guardJoiner(items, joiners, token, rawQuery)
                        joiners.add(token.value.toString())
                        offset++
                    }

                    else -> break
                }
            }

            // Also check for postfix indexer after resolving
            val result = resolvePrecedence(items, joiners)

            // Check for trailing indexer
            if (offset < tokens.size && tokens[offset].type == TokenType.SPECIAL && tokens[offset].value == "[") {
                val (indexApp, newOffset) = consumeIndexer(tokens, offset, rawQuery)
                val indexCall = indexApp as Expression.FnCall
                return Pair(Expression.FnCall(indexCall.function, indexCall.arguments + result), newOffset)
            }

            return Pair(result, offset)
        }

        private fun applyUnaries(expr: Expression, processors: List<(Expression) -> Expression>): Expression {
            var result = expr
            for (proc in processors.reversed()) {
                result = proc(result)
            }
            return result
        }

        private fun guardItem(items: List<Expression>, joiners: List<String>, token: Token, rawQuery: String) {
            if (joiners.size != items.size) {
                throw MistQLError.Parse("Unexpected token ${token.value}")
            }
        }

        private fun guardJoiner(items: List<Expression>, joiners: List<String>, token: Token, rawQuery: String) {
            if (joiners.size + 1 != items.size) {
                throw MistQLError.Parse("Unexpected token ${token.value}")
            }
        }

        private fun valueLiteral(value: Any?): Expression {
            return when (value) {
                null -> Expression.Value(RuntimeValue.Null)
                is Boolean -> Expression.Value(RuntimeValue.Bool(value))
                is Double -> Expression.Value(RuntimeValue.Num(value))
                is String -> Expression.Value(RuntimeValue.Str(value))
                else -> throw MistQLError.Parse("Unknown value type: $value")
            }
        }

        // ────── Precedence resolution ──────

        private fun resolvePrecedence(items: List<Expression>, joiners: List<String>): Expression {
            if (items.isEmpty()) throw MistQLError.Parse("Tried to parse empty expression!")
            if (items.size == 1 && joiners.isEmpty()) return items[0]
            if (items.size - 1 != joiners.size) throw MistQLError.Parse("Invalid binary expression sequence!")

            var currentItems = items.toMutableList()
            var currentJoiners = joiners.toMutableList()

            // Stage 1: Simple binary operators (highest precedence first)
            for (precedenceLevel in SIMPLE_BINARY_OPERATORS) {
                val newItems = mutableListOf(currentItems[0])
                val newJoiners = mutableListOf<String>()

                for (j in currentJoiners.indices) {
                    newItems.add(currentItems[j + 1])
                    if (currentJoiners[j] in precedenceLevel) {
                        val right = newItems.removeLast()
                        val left = newItems.removeLast()
                        val op = currentJoiners[j]
                        newItems.add(Expression.FnCall(
                            Expression.Ref(op, internal = true),
                            listOf(left, right)
                        ))
                    } else {
                        newJoiners.add(currentJoiners[j])
                    }
                }
                currentItems = newItems
                currentJoiners = newJoiners
            }

            // Stage 2: Amalgamating operators (space, then pipe)
            for (amalgOp in AMALGAMATING_OPERATORS) {
                val newItems = mutableListOf(currentItems[0])
                val newJoiners = mutableListOf<String>()
                var streak = mutableListOf<Expression>()

                val flushStreak = {
                    if (streak.isNotEmpty()) {
                        newItems.add(amalgamate(amalgOp, streak))
                        streak = mutableListOf()
                    }
                }

                for (j in currentJoiners.indices) {
                    if (currentJoiners[j] == amalgOp) {
                        if (streak.isEmpty()) {
                            streak.add(currentItems[j])
                            newItems.removeLast()
                        }
                        streak.add(currentItems[j + 1])
                    } else {
                        flushStreak()
                        newItems.add(currentItems[j + 1])
                        newJoiners.add(currentJoiners[j])
                    }
                }
                flushStreak()

                currentItems = newItems
                currentJoiners = newJoiners
            }

            if (currentJoiners.isNotEmpty()) {
                throw MistQLError.Parse("Expected expression following binary expression ${currentJoiners[0]}")
            }
            return currentItems[0]
        }

        private fun amalgamate(op: String, items: List<Expression>): Expression {
            return when (op) {
                " " -> {
                    // Function application: first item is function, rest are arguments
                    Expression.FnCall(items[0], items.subList(1, items.size), isExplicitCall = true)
                }
                "|" -> {
                    // Pipeline: wrap non-explicit-call stages
                    val stages = mutableListOf(items[0])
                    for (i in 1 until items.size) {
                        val expr = items[i]
                        if (expr is Expression.FnCall && expr.isExplicitCall) {
                            // Explicit function call (from space) — keep as-is
                            stages.add(expr)
                        } else {
                            // Non-explicit (operator result, bare ref, etc.) — wrap in zero-arg application
                            stages.add(Expression.FnCall(expr, emptyList(), isExplicitCall = true))
                        }
                    }
                    Expression.Pipe(stages)
                }
                else -> throw MistQLError.Parse("Unknown amalgamating operator: $op")
            }
        }

        // ────── Composite parsers ──────

        private fun consumeParenthetical(tokens: List<Token>, startOffset: Int, rawQuery: String): Pair<Expression, Int> {
            var offset = startOffset + 1 // skip (
            val (result, newOffset) = consumeExpression(tokens, offset, rawQuery)
            offset = newOffset
            if (offset >= tokens.size || tokens[offset].value != ")") {
                throw MistQLError.Parse("Expected ')'")
            }
            offset++
            return Pair(Expression.Paren(result), offset)
        }

        private fun consumeArray(tokens: List<Token>, startOffset: Int, rawQuery: String): Pair<Expression, Int> {
            var offset = startOffset + 1 // skip [
            val entries = mutableListOf<Expression>()
            while (true) {
                if (offset < tokens.size && tokens[offset].value == "]") {
                    offset++
                    break
                }
                val (result, newOffset) = consumeExpression(tokens, offset, rawQuery)
                entries.add(result)
                offset = newOffset
                if (offset < tokens.size && tokens[offset].value == ",") {
                    offset++
                    continue
                } else if (offset < tokens.size && tokens[offset].value == "]") {
                    offset++
                    break
                } else {
                    throw MistQLError.Parse("Expected ',' or ']'")
                }
            }
            return Pair(Expression.ArrayLiteral(entries), offset)
        }

        private fun consumeStruct(tokens: List<Token>, startOffset: Int, rawQuery: String): Pair<Expression, Int> {
            var offset = startOffset + 1 // skip {
            val entries = mutableListOf<Pair<String, Expression>>()
            while (true) {
                if (offset < tokens.size && tokens[offset].value == "}") {
                    offset++
                    break
                }
                if (offset >= tokens.size) throw MistQLError.Parse("Unexpected EOF")

                // Key: ref or string value
                val key: String = when {
                    tokens[offset].type == TokenType.REF -> tokens[offset].value.toString()
                    tokens[offset].type == TokenType.VALUE && tokens[offset].value is String -> tokens[offset].value.toString()
                    else -> throw MistQLError.Parse("Expected object key, got ${tokens[offset].value}")
                }
                offset++

                // Colon
                if (offset >= tokens.size || tokens[offset].value != ":") {
                    throw MistQLError.Parse("Expected ':'")
                }
                offset++

                // Value
                val (value, newOffset) = consumeExpression(tokens, offset, rawQuery)
                entries.add(Pair(key, value))
                offset = newOffset

                if (offset < tokens.size && tokens[offset].value == ",") {
                    offset++
                    continue
                } else if (offset < tokens.size && tokens[offset].value == "}") {
                    offset++
                    break
                } else {
                    throw MistQLError.Parse("Expected ',' or '}'")
                }
            }
            return Pair(Expression.ObjectLiteral(entries), offset)
        }

        private fun consumeDotAccess(left: Expression, tokens: List<Token>, startOffset: Int, rawQuery: String): Pair<Expression, Int> {
            var offset = startOffset + 1 // skip .
            if (offset >= tokens.size || tokens[offset].type != TokenType.REF) {
                throw MistQLError.Parse("Expected identifier after '.'")
            }
            val name = tokens[offset].value.toString()
            offset++
            return Pair(
                Expression.FnCall(
                    Expression.Ref(".", internal = true),
                    listOf(left, Expression.Ref(name))
                ),
                offset
            )
        }

        private fun consumeIndexer(tokens: List<Token>, startOffset: Int, rawQuery: String): Pair<Expression, Int> {
            var offset = startOffset + 1 // skip [
            val entries = mutableListOf<Expression>()
            while (true) {
                if (offset < tokens.size && tokens[offset].value == ":") {
                    offset++
                    entries.add(Expression.Value(RuntimeValue.Null))
                    continue
                } else if (offset < tokens.size && tokens[offset].value == "]") {
                    offset++
                    entries.add(Expression.Value(RuntimeValue.Null))
                    break
                }
                val (result, newOffset) = consumeExpression(tokens, offset, rawQuery)
                entries.add(result)
                offset = newOffset
                if (offset < tokens.size && tokens[offset].value == ":") {
                    offset++
                    continue
                } else if (offset < tokens.size && tokens[offset].value == "]") {
                    offset++
                    break
                } else {
                    throw MistQLError.Parse("Expected ':', ']'")
                }
            }
            return Pair(
                Expression.FnCall(Expression.Ref("index", internal = true), entries),
                offset
            )
        }
    }

    // ────────── Lexer ──────────

    enum class TokenType { VALUE, REF, SPECIAL }
    data class Token(val type: TokenType, val value: Any?, val position: Int)

    private fun lex(): List<Token> {
        val tokens = mutableListOf<Token>()
        while (pos < input.length) {
            val position = pos
            val c = input[pos]

            when {
                // Number
                c.isDigit() -> {
                    val numStr = input.substring(pos)
                    val matchResult = NUM_REGEX.find(numStr)
                        ?: throw MistQLError.Parse("Invalid number at position $pos")
                    val matchStr = matchResult.value
                    tokens.add(Token(TokenType.VALUE, matchStr.toDouble(), position))
                    pos += matchStr.length
                }

                // Whitespace
                c.isWhitespace() -> {
                    while (pos < input.length && input[pos].isWhitespace()) pos++
                    tokens.add(Token(TokenType.SPECIAL, " ", position))
                }

                // String
                c == '"' -> {
                    tokens.add(Token(TokenType.VALUE, parseString(), position))
                }

                // @ or $
                c == '@' || c == '$' -> {
                    tokens.add(Token(TokenType.REF, c.toString(), position))
                    pos++
                }

                // Identifier
                c.isLetter() || c == '_' -> {
                    val start = pos
                    pos++
                    while (pos < input.length && (input[pos].isLetterOrDigit() || input[pos] == '_')) pos++
                    val name = input.substring(start, pos)
                    if (name in BUILTIN_VALUES) {
                        tokens.add(Token(TokenType.VALUE, BUILTIN_VALUES[name], position))
                    } else {
                        tokens.add(Token(TokenType.REF, name, position))
                    }
                }

                // Special characters (operators, punctuation)
                else -> {
                    // Try matching longest special first
                    var matched: String? = null
                    for (special in SPECIALS.sortedByDescending { it.length }) {
                        if (input.startsWith(special, pos)) {
                            matched = special
                            break
                        }
                    }
                    if (matched != null) {
                        // Handle whitespace vacuuming
                        if (vacuumsWhitespace(matched, "r")) {
                            pos += matched.length
                            while (pos < input.length && input[pos].isWhitespace()) pos++
                        } else {
                            pos += matched.length
                        }
                        if (vacuumsWhitespace(matched, "l") && tokens.isNotEmpty()
                            && tokens.last().type == TokenType.SPECIAL && tokens.last().value == " ") {
                            tokens.removeLast()
                        }
                        tokens.add(Token(TokenType.SPECIAL, matched, position))
                    } else {
                        throw MistQLError.Parse("Unexpected character '$c' at position $pos")
                    }
                }
            }
        }

        // Trim leading/trailing whitespace tokens
        if (tokens.isNotEmpty() && tokens.first().type == TokenType.SPECIAL && tokens.first().value == " ") {
            tokens.removeFirst()
        }
        if (tokens.isNotEmpty() && tokens.last().type == TokenType.SPECIAL && tokens.last().value == " ") {
            tokens.removeLast()
        }

        return tokens
    }

    private fun parseString(): String {
        pos++ // consume opening '"'
        val sb = StringBuilder()
        while (pos < input.length) {
            val c = input[pos]
            if (c == '"') {
                pos++
                return sb.toString()
            }
            if (c == '\\') {
                pos++
                if (pos >= input.length) throw MistQLError.Parse("Unterminated string escape")
                when (val escaped = input[pos]) {
                    '"' -> sb.append('"')
                    '\\' -> sb.append('\\')
                    '/' -> sb.append('/')
                    'b' -> sb.append('\b')
                    'f' -> sb.append('\u000C')
                    'n' -> sb.append('\n')
                    'r' -> sb.append('\r')
                    't' -> sb.append('\t')
                    'u' -> {
                        if (pos + 4 >= input.length) throw MistQLError.Parse("Invalid unicode escape")
                        val hex = input.substring(pos + 1, pos + 5)
                        sb.append(hex.toInt(16).toChar())
                        pos += 4
                    }
                    else -> {
                        sb.append('\\')
                        sb.append(escaped)
                    }
                }
            } else {
                sb.append(c)
            }
            pos++
        }
        throw MistQLError.Parse("Unterminated string literal")
    }

}
