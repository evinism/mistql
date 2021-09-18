import { ASTExpression, LexToken } from "./types";
import { escapeRegExp } from "./util";
/*
type Update = {
  append: boolean,
  token: LexToken,
};

type TokenDomain = 'alphaunder' | 'number';

const reserved = [

];

const domain = (char: string): TokenDomain => {
  if (/[a-zA-Z]/.test(char)) {
    return 'alphaunder';
  }
  if (/[0-9]/.test(char)) {
    return 'number';
  }
  if () {

  }
  throw new Error("Lexing Error");
}

const combiner = (prevToken?: LexToken, char: string): Update => {
  const 
  for (let i = 0; )
}
*/

const refStarter = /[a-zA-Z_]/;
const refContinuer = /[a-zA-Z_0-9]/;
const numStarter = /[0-9]/;
const numContinuer = /[0-9\.]/;

// These binary expressions have definitions that span over
// n consecutive items, e.g. function application. Could absolutely be extended
// to all commutative operators, e.g. +, *, &&, and ||. For simplicity, though, pls no.
const amalgamatingBinaryOperators = [" ", "|"];

const amalgamationTechniques: {
  [key: string]: (start: ASTExpression[]) => ASTExpression;
} = {
  " ": (asts) => ({
    type: "application",
    function: asts[0],
    arguments: asts.slice(1),
  }),
  "|": (asts) => ({
    type: "pipeline",
    stages: asts,
  }),
};

const simpleBinaryOperators = [
  ".",
  "*",
  "/",
  "%",
  "+",
  "-",
  "<",
  ">",
  "<=",
  ">=",
  "==",
  "!=",
  "&&",
  "||",
];

const binaryExpressionStrings = [].concat(
  simpleBinaryOperators,
  amalgamatingBinaryOperators
);

const specials = binaryExpressionStrings.concat(["(", ")", "[", "]", ","]);

const unaryExpressions = ["-", "!"];

const builtinValues = {
  true: true,
  false: false,
  null: null,
};

const whiteSpacealyzer = (str: string) => {
  // TODO: Make these more solid.
  let retval = str
    .trim()
    .replace(/\s+/g, " ")
    .replace(/\s*\.\s*/g, ".")
    .replace(/\(\s*/g, "(")
    .replace(/\s*\)/g, ")")
    .replace(/\s*\|\s*/g, "|")
    .replace(/\s*,\s*/g, ",");
  binaryExpressionStrings.forEach((binexp) => {
    const re = new RegExp(`\\s*${escapeRegExp(binexp)}\\s*`, "g");
    retval = retval.replace(re, binexp);
  });
  return retval;
};

export function lexer(raw: string): LexToken[] {
  const tokens: LexToken[] = [];
  const split = whiteSpacealyzer(raw).split("");
  for (let i = 0; i < split.length; i++) {
    let buffer = split[i];
    if (numStarter.test(buffer || "")) {
      while (numContinuer.test(split[i + 1] || "")) {
        i++;
        buffer += split[i];
      }
      tokens.push({ token: "value", value: parseFloat(buffer) });
    } else if (
      specials.filter((operator) => operator.startsWith(buffer)).length > 0
    ) {
      while (
        specials.filter((operator) =>
          operator.startsWith(buffer + split[i + 1])
        ).length > 0
      ) {
        i++;
        buffer += split[i];
      }
      tokens.push({ token: "special", value: buffer });
    } else if (refStarter.test(buffer || "")) {
      while (refContinuer.test(split[i + 1] || "")) {
        i++;
        buffer += split[i];
      }
      if (builtinValues[buffer] !== undefined) {
        tokens.push({ token: "value", value: builtinValues[buffer] });
      } else {
        tokens.push({ token: "ref", value: buffer });
      }
    } else if (buffer === "@") {
      tokens.push({ token: "ref", value: "@" });
    } else if (buffer === '"') {
      buffer = "";
      while (split[i + 1] !== '"') {
        i++;
        if (split[i] === "\\") {
          i++;
        }
        buffer += split[i];
      }
      tokens.push({ token: "value", value: buffer });
      i++;
    } else {
      throw new Error("Lexer Error");
    }
  }
  return tokens;
}

type ParseResult = {
  result: ASTExpression;
  remaining: LexToken[];
};

type Parser = (tokens: LexToken[]) => ParseResult;

const tmatch = (token: string, value: unknown, root: LexToken) => {
  return root.token === token && root.value === value;
};

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

const consumeParenthetical: Parser = (tokens: LexToken[]) => {
  let current = tokens;
  if (!tmatch("special", "(", current[0])) {
    throw new Error("Parenthetical Issue");
  }
  current = current.slice(1);
  const { result, remaining } = consumeExpression(current);
  current = remaining;
  if (!current[0]) {
    throw new Error("Unexpected EOF");
  }
  if (!tmatch("special", ")", current[0])) {
    throw new Error("Expected )");
  }
  current = current.slice(1);
  return {
    result,
    remaining: current,
  };
};

const consumeArray: Parser = (tokens) => {
  if (!tmatch("special", "[", tokens[0])) {
    throw new Error("ParenStart Issue");
  }
  let current = tokens.slice(1);
  let entries: ASTExpression[] = [];
  // dirty explicit check for an empty array -- should be fixed up
  if (!(current[0] && tmatch("special", "]", current[0]))) {
    while (true) {
      const { result, remaining } = consumeExpression(current);
      entries.push(result);
      current = remaining;
      if (tmatch("special", ",", current[0])) {
        current = current.slice(1);
        continue;
      } else if (tmatch("special", "]", current[0])) {
        current = current.slice(1);
        break;
      } else {
        throw new Error("Unexpected Token " + current[0].value);
      }
    }
  } else {
    current = current.slice(1);
  }
  return {
    result: {
      type: "literal",
      valueType: "array",
      value: entries,
    },
    remaining: current,
  };
};

// This might be the worst function i've ever written.
// But at least it's a contained transformation.
type BinaryExpressionSequence = { items: ASTExpression[]; joiners: string[] };

const turnBinaryExpressionSequenceIntoASTExpression = (
  bexpseq: BinaryExpressionSequence
): ASTExpression => {
  if (bexpseq.items.length === 0) {
    throw new Error("Can't join 0 items!");
  }
  if (bexpseq.items.length === 1) {
    // this is the majority case by a long shot.
    return bexpseq.items[0];
  }
  let current = bexpseq;

  // First Stage: Simple Binary Expressions -> Applications
  for (let i = 0; i < simpleBinaryOperators.length; i++) {
    const currentExpression = simpleBinaryOperators[i];
    const newItems = [current.items[0]];
    const newJoiners = [];

    for (let j = 0; j < current.joiners.length; j++) {
      newItems.push(current.items[j + 1]);
      if (current.joiners[j] === currentExpression) {
        const l = newItems[newItems.length - 2];
        const r = newItems[newItems.length - 1];
        newItems[newItems.length - 2] = {
          type: "application",
          function: {
            type: "reference",
            ref: currentExpression,
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
  return current.items[0];
};

const consumeExpression: Parser = (tokens) => {
  let current = tokens;
  let items: ASTExpression[] = [];
  let joiners: LexToken[] = [];

  const itemPushGuard = (token: LexToken) => {
    if (joiners.length !== items.length) {
      // Now parsing an item, so guard
      throw new Error("Unexpected Token " + token.value);
    }
  };

  const binExpDoesntMakeSense = () => {
    return joiners.length + 1 !== items.length;
  };

  const joinerPushGuard = (token: LexToken) => {
    if (binExpDoesntMakeSense()) {
      // Now parsing an item, so guard
      throw new Error(" zzUnexpected Token " + token.value);
    }
  };
  while (current.length > 0) {
    let next = current[0];

    // --- NASTY HACK ALERT ---
    // Weird dirty hack that should be sorted out.
    // only if binary expression WOULD throw, parse as a unary as a "backup"
    let hackyUnaryPostProcess:
      | ((ast: ASTExpression) => ASTExpression)
      | undefined = undefined;
    if (isUnExp(next) && binExpDoesntMakeSense()) {
      // turn all further unaries into a big ol' stack.
      let i = 0; // index of first non-unary item.
      for (; i < current.length; i++) {
        if (!isUnExp(current[i])) {
          break;
        }
      }
      const unaries = current.slice(0, i);
      current = current.slice(i);
      next = current[0];
      hackyUnaryPostProcess = (item) =>
        unaries.reduce(
          (acc, cur) => ({
            type: "application",
            function: {
              type: "reference",
              ref: (cur.value as string) + "/unary",
            },
            arguments: [acc],
          }),
          item
        );
    }

    if (isBinExp(next) && !hackyUnaryPostProcess) {
      joinerPushGuard(next);
      joiners.push(next);
      current = current.slice(1);
    } else if (tmatch("special", "(", next)) {
      itemPushGuard(next);
      const { result, remaining } = consumeParenthetical(current);
      items.push(result);
      current = remaining;
    } else if (tmatch("special", "[", next)) {
      itemPushGuard(next);
      const { result, remaining } = consumeArray(current);
      items.push(result);
      current = remaining;
    } else if (next.token === "value") {
      itemPushGuard(next);
      items.push({
        type: "literal",
        valueType: next.value !== null ? (typeof next.value as any) : "null",
        value: next.value,
      });
      current = current.slice(1);
    } else if (next.token === "ref") {
      itemPushGuard(next);
      items.push({
        type: "reference",
        ref: next.value,
      });
      current = current.slice(1);
    } else {
      break;
      // An unexpected token! Stop parsing this expression
    }
    // This is incredibly gross
    if (hackyUnaryPostProcess) {
      items[items.length - 1] = hackyUnaryPostProcess(items[items.length - 1]);
    }
  }

  return {
    result: turnBinaryExpressionSequenceIntoASTExpression({
      items,
      // We know the below is a string because we only add specials
      joiners: joiners.map((joiner) => joiner.value as string),
    }),
    remaining: current.slice(),
  };
};

function parseQuery(tokens: LexToken[]): ASTExpression {
  const { result, remaining } = consumeExpression(tokens);
  if (remaining.length !== 0) {
    throw new Error("Unexpected token " + remaining[0].value);
  }
  return result;
}

function parse(raw: string): ASTExpression {
  const lexed = lexer(raw);
  const parsed = parseQuery(lexed);
  return parsed;
}

export const parseOrThrow = parse;
