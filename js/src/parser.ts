import {
  amalgamatingBinaryOperators,
  binaryExpressionStrings,
  simpleBinaryOperators,
  unaryExpressions
} from "./constants";
import { OpenAnIssueIfThisOccursError, ParseError, UnpositionableParseError } from "./errors";
import { lex } from "./lexer";
import { ASTApplicationExpression, ASTExpression, LexToken } from "./types";

/*
 * To all who dare enter:
 * The parser below is a maelstrom of spaghetti, but is very small.
 * Please refactor it if you can, but keep bundle size down to a minimum.
 */

const amalgamationTechniques: {
  [key: string]: (start: ASTExpression[]) => ASTExpression;
} = {
  " ": (asts) => ({
    type: "application",
    function: asts[0],
    arguments: asts.slice(1),
    _shouldntWrapInPipedExpressions: true,
  }),
  "|": (asts) => ({
    type: "pipeline",
    stages: [].concat(
      asts.slice(0, 1),
      asts.slice(1).map((expr) => {
        if (
          expr.type === "application" &&
          expr._shouldntWrapInPipedExpressions
        ) {
          return expr;
        }
        return {
          type: "application",
          function: expr,
          arguments: [],
        };
      })
    ),
  }),
};

type ParseResult = {
  result: ASTExpression;
  offset: number;
};

type ParseContext = {
  rawQuery: string;
};

type Parser = (
  tokens: LexToken[],
  offset: number,
  ctx: ParseContext
) => ParseResult;
type ParameterizedParser = (
  sourceItem: ASTExpression,
  tokens: LexToken[],
  offset: number,
  ctx: ParseContext
) => ParseResult;

const tmatch = (token: string, value: unknown, root: LexToken) => {
  return root !== undefined && root.token === token && root.value === value;
};

const buildTMatchThrower =
  (ErrorConstructor: any) =>
  (token: string, value: unknown, root: LexToken) => {
    if (!tmatch(token, value, root)) {
      throw new ErrorConstructor("Expected " + value + ", got " + root.value);
    }
  };

const tmatchOrThrow = buildTMatchThrower(ParseError);
const tmatchOrThrowBad = buildTMatchThrower(OpenAnIssueIfThisOccursError);

const isBinExp = (token: LexToken) => {
  const res =
    token.token === "special" &&
    binaryExpressionStrings.indexOf(token.value) > -1;
  return res;
};

const isUnExp = (token: LexToken) => {
  const res =
    token.token === "special" && unaryExpressions.indexOf(token.value) > -1;
  return res;
};

const consumeParenthetical: Parser = (tokens, offset, ctx) => {
  tmatchOrThrowBad("special", "(", tokens[offset]);
  offset++;
  const { result, offset: nextOffset } = consumeExpression(tokens, offset, ctx);
  offset = nextOffset;
  if (!tokens[offset]) {
    throw new ParseError(
      "Unexpected EOF",
      ctx.rawQuery.length - 1,
      ctx.rawQuery
    );
  }
  tmatchOrThrow("special", ")", tokens[offset]);
  offset++;
  return {
    result: {
      type: "parenthetical",
      expression: result,
    },
    offset,
  };
};

const consumeArray: Parser = (tokens, offset, ctx) => {
  tmatchOrThrowBad("special", "[", tokens[offset]);
  offset++;
  let entries: ASTExpression[] = [];
  // dirty explicit check for an empty array -- should be fixed up
  while (true) {
    if (tmatch("special", "]", tokens[offset])) {
      offset++;
      break;
    }
    const { result, offset: newOffset } = consumeExpression(
      tokens,
      offset,
      ctx
    );
    entries.push(result);
    offset = newOffset;
    if (tmatch("special", ",", tokens[offset])) {
      offset++;
      continue;
    } else if (tmatch("special", "]", tokens[offset])) {
      offset++;
      break;
    } else {
      throw new ParseError(
        "Unexpected token " + tokens[offset].value,
        tokens[offset].position,
        ctx.rawQuery
      );
    }
  }
  return {
    result: {
      type: "literal",
      valueType: "array",
      value: entries,
    },
    offset,
  };
};

const consumeIndexer: Parser = (tokens, offset, ctx) => {
  tmatchOrThrowBad("special", "[", tokens[offset]);
  offset++;
  let entries: ASTExpression[] = [];
  while (true) {
    // This could be simplified dramaticallly.
    if (tmatch("special", ":", tokens[offset])) {
      offset++;
      entries.push({
        type: "literal",
        valueType: "null",
        value: null,
      });
      continue;
    } else if (tmatch("special", "]", tokens[offset])) {
      offset++;
      entries.push({
        type: "literal",
        valueType: "null",
        value: null,
      });
      break;
    }
    const { result, offset: newOffset } = consumeExpression(
      tokens,
      offset,
      ctx
    );
    entries.push(result);
    offset = newOffset;
    if (tmatch("special", ":", tokens[offset])) {
      offset++;
      continue;
    } else if (tmatch("special", "]", tokens[offset])) {
      offset++;
      break;
    } else {
      throw new ParseError(
        "Unexpected token " + tokens[offset].value,
        tokens[offset].position,
        ctx.rawQuery
      );
    }
  }

  const app: ASTExpression = {
    type: "application",
    function: {
      type: "reference",
      ref: "index",
      internal: true,
    },
    arguments: entries,
  };
  return {
    result: app,
    offset,
  };
};

const consumeStruct: Parser = (tokens, offset, ctx) => {
  tmatchOrThrowBad("special", "{", tokens[offset]);
  offset++;
  let entries: { [key: string]: ASTExpression } = {};
  while (true) {
    if (tmatch("special", "}", tokens[offset])) {
      offset++;
      break;
    }
    if (tokens[offset] === undefined) {
      throw new ParseError("Unexpected EOF", ctx.rawQuery.length, ctx.rawQuery);
    }
    let key: string;
    if (tokens[offset].token === "ref" || tokens[offset].token === "value") {
      key = tokens[offset].value.toString();
      offset++;
    } else {
      throw new ParseError(
        "Unexpected token " + tokens[offset].value,
        tokens[offset].position,
        ctx.rawQuery
      );
    }
    tmatchOrThrow("special", ":", tokens[offset]);
    offset++;
    const { result, offset: newOffset } = consumeExpression(
      tokens,
      offset,
      ctx
    );
    offset = newOffset;
    entries[key] = result;
    if (tmatch("special", ",", tokens[offset])) {
      offset++;
      continue;
    } else if (tmatch("special", "}", tokens[offset])) {
      offset++;
      break;
    } else {
      throw new ParseError(
        "Unexpected token " + tokens[offset].value,
        tokens[offset].position,
        ctx.rawQuery
      );
    }
  }
  return {
    result: {
      type: "literal",
      valueType: "object",
      value: entries,
    },
    offset,
  };
};

const consumeDotAccess: ParameterizedParser = (left, tokens, offset, ctx) => {
  tmatchOrThrowBad("special", ".", tokens[offset]);
  offset++;
  let ref: string;
  let refToken = tokens[offset];
  if (!refToken || refToken.token !== "ref") {
    throw new ParseError(
      "Unexpected token " + tokens[offset].value + ", expected :",
      tokens[offset].position,
      ctx.rawQuery
    );
  }
  ref = refToken.value;
  offset++;
  const result: ASTExpression = {
    type: "application",
    function: {
      type: "reference",
      ref: ".",
      internal: true,
    },
    arguments: [left, { type: "reference", ref: ref }],
  };
  return { result, offset };
};

// This might be the worst function i've ever written.
// But at least it's a contained transformation.
type BinaryExpressionSequence = { items: ASTExpression[]; joiners: string[] };

const turnBinaryExpressionSequenceIntoASTExpression = (
  bexpseq: BinaryExpressionSequence
): ASTExpression => {
  if (bexpseq.items.length === 0) {
    throw new UnpositionableParseError("Tried to parse empty expression!");
  }
  if (bexpseq.items.length === 1 && bexpseq.joiners.length === 0) {
    // this is the majority case by a long shot.
    return bexpseq.items[0];
  }
  if (bexpseq.items.length - 1 !== bexpseq.joiners.length) {
    throw new UnpositionableParseError(
      "Invalid sequence of binary expressions!"
    );
  }
  let current = bexpseq;

  // First Stage: Simple Binary Expressions -> Applications
  for (let i = 0; i < simpleBinaryOperators.length; i++) {
    const currentPrecedenceLevel = simpleBinaryOperators[i];
    const newItems = [current.items[0]];
    const newJoiners = [];

    for (let j = 0; j < current.joiners.length; j++) {
      newItems.push(current.items[j + 1]);
      if (currentPrecedenceLevel.indexOf(current.joiners[j]) > -1) {
        const l = newItems[newItems.length - 2];
        const r = newItems[newItems.length - 1];
        newItems[newItems.length - 2] = {
          type: "application",
          function: {
            type: "reference",
            ref: current.joiners[j],
            internal: true,
          },
          arguments: [l, r],
        };
        newItems.length = newItems.length - 1;
      } else {
        newJoiners.push(current.joiners[j]);
      }
    }
    current = {
      items: newItems,
      joiners: newJoiners,
    };
  }

  // Second Stage: Amalgamating Binary Expressions
  for (let i = 0; i < amalgamatingBinaryOperators.length; i++) {
    const currentExpression = amalgamatingBinaryOperators[i];
    const newItems = [current.items[0]];
    const newJoiners = [];
    const amalgamationTechnique = amalgamationTechniques[currentExpression]!;
    let streak: ASTExpression[] = [];
    const flushStreak = () => {
      if (streak.length > 0) {
        newItems.push(amalgamationTechnique(streak));
        streak = [];
      }
    };
    for (let j = 0; j < current.joiners.length; j++) {
      if (current.joiners[j] === currentExpression) {
        if (streak.length === 0) {
          streak.push(current.items[j]);
          newItems.pop();
        }
        streak.push(current.items[j + 1]);
      } else {
        // Flush the current streak.
        flushStreak();
        newItems.push(current.items[j + 1]);
        newJoiners.push(current.joiners[j]);
      }
    }
    flushStreak();
    current = {
      items: newItems,
      joiners: newJoiners,
    };
  }
  if (current.joiners.length !== 0) {
    throw new UnpositionableParseError(
      "Expected expression following binary expression " + current.joiners[0]
    );
  }
  return current.items[0];
};

const consumeExpression: Parser = (tokens, offset, ctx) => {
  let items: ASTExpression[] = [];
  let joiners: LexToken[] = [];

  const itemPushGuard = (token: LexToken) => {
    if (joiners.length !== items.length) {
      // Now parsing an item, so guard
      throw new ParseError(
        "Unexpected Token " + token.value,
        token.position,
        ctx.rawQuery
      );
    }
  };

  const binExpDoesntMakeSense = () => {
    return joiners.length + 1 !== items.length;
  };

  const joinerPushGuard = (token: LexToken) => {
    if (binExpDoesntMakeSense()) {
      // Now parsing an item, so guard
      throw new ParseError(
        "Unexpected Token " + token.value,
        token.position,
        ctx.rawQuery
      );
    }
  };
  while (offset < tokens.length) {
    let next = tokens[offset];

    // --- NASTY HACK ALERT ---
    // Weird dirty hack that should be sorted out.
    // only if binary expression WOULD throw, parse as a unary as a "backup"
    let hackyUnaryPostProcess:
      | ((ast: ASTExpression) => ASTExpression)
      | undefined = undefined;
    if (isUnExp(next) && binExpDoesntMakeSense()) {
      // turn all further unaries into a big ol' stack.
      let i = offset; // index of first non-unary item.
      for (; i < tokens.length; i++) {
        if (!isUnExp(tokens[i])) {
          break;
        }
      }
      const unaries = tokens.slice(offset, i);
      offset = i;
      next = tokens[offset];
      hackyUnaryPostProcess = (item) =>
        unaries.reduceRight(
          (acc, cur) => ({
            type: "application",
            function: {
              type: "reference",
              ref: (cur.value as string) + "/unary",
              internal: true,
            },
            arguments: [acc],
          }),
          item
        );
    }

    if (tmatch("special", "(", next)) {
      itemPushGuard(next);
      const { result, offset: newOffset } = consumeParenthetical(
        tokens,
        offset,
        ctx
      );
      items.push(result);
      offset = newOffset;
    } else if (tmatch("special", ".", next)) {
      if (items.length === 0) {
        throw new ParseError("Unexpected Token .", next.position, ctx.rawQuery);
      }
      const { result, offset: newOffset } = consumeDotAccess(
        items.pop(),
        tokens,
        offset,
        ctx
      );
      items.push(result);
      offset = newOffset;
    } else if (tmatch("special", "[", next)) {
      // If it doesn't make sense as an item, then it should be parsed
      // as an indexing expression instead!!
      if (joiners.length === items.length) {
        const { result, offset: newOffset } = consumeArray(tokens, offset, ctx);
        items.push(result);
        offset = newOffset;
      } else {
        // We can always postfix an expression with an indexing term. Binds at maximum depth.
        // Replaces the previous
        const { result: app, offset: newOffset } = consumeIndexer(
          tokens,
          offset,
          ctx
        );
        items[items.length - 1] = {
          type: "application",
          function: (app as ASTApplicationExpression).function,
          arguments: (app as ASTApplicationExpression).arguments.concat([
            items[items.length - 1],
          ]),
        };
        offset = newOffset;
      }
    } else if (tmatch("special", "{", next)) {
      itemPushGuard(next);
      const { result, offset: newOffset } = consumeStruct(tokens, offset, ctx);
      items.push(result);
      offset = newOffset;
    } else if (next.token === "value") {
      itemPushGuard(next);
      items.push({
        type: "literal",
        valueType: next.value !== null ? (typeof next.value as any) : "null",
        value: next.value,
      });
      offset++;
    } else if (next.token === "ref") {
      itemPushGuard(next);
      items.push({
        type: "reference",
        ref: next.value,
      });
      offset++;
    } else if (isBinExp(next) && !hackyUnaryPostProcess) {
      joinerPushGuard(next);
      joiners.push(next);
      offset++;
    } else {
      break;
      // An unexpected token! Stop parsing this expression
    }
    // This is incredibly gross
    if (hackyUnaryPostProcess) {
      items[items.length - 1] = hackyUnaryPostProcess(items[items.length - 1]);
    }
  }

  let resolvedSequence = turnBinaryExpressionSequenceIntoASTExpression({
    items,
    // We know the below is a string because we only add specials
    joiners: joiners.map((joiner) => joiner.value as string),
  });

  // We can always postfix an expression with an indexing term. Binds at maximum depth.
  if (tmatch("special", "[", tokens[offset])) {
    const { result: app, offset: nextOffset } = consumeIndexer(
      tokens,
      offset,
      ctx
    );
    resolvedSequence = {
      type: "application",
      function: (app as ASTApplicationExpression).function,
      arguments: (app as ASTApplicationExpression).arguments.concat([
        resolvedSequence,
      ]),
    };
    offset = nextOffset;
  }

  return {
    result: turnBinaryExpressionSequenceIntoASTExpression({
      items,
      // We know the below is a string because we only add specials
      joiners: joiners.map((joiner) => joiner.value as string),
    }),
    offset,
  };
};

function parseQuery(tokens: LexToken[], ctx: ParseContext): ASTExpression {
  const { result, offset } = consumeExpression(tokens, 0, ctx);
  if (offset !== tokens.length) {
    throw new ParseError(
      "Unexpected token " + tokens[offset].value + ", expected EOF",
      ctx.rawQuery.length,
      ctx.rawQuery
    );
  }
  return result;
}

function parse(raw: string): ASTExpression {
  const lexed = lex(raw);
  const ctx: ParseContext = {
    rawQuery: raw,
  }
  const parsed = parseQuery(lexed, ctx);
  return parsed;
}

export const parseOrThrow = parse;
