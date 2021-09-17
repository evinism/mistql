import { ASTExpression, LexToken } from './types';
import { escapeRegExp } from './util';
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
const binaryExpressionStrings = [
  ".",
  "*",
  "/",
  "%",
  "+",
  "-",
  "<=",
  ">=",
  "<",
  ">",
  "==",
  "!=",
  "&&",
  "||",
  " ",

];

const specials = binaryExpressionStrings.concat([
  '(',
  ')',
  '[',
  ']',
  ',',
  "|"
]);

const builtinValues = {
  true: true,
  false: false,
  null: null,
}

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
  const split = whiteSpacealyzer(raw).split('');
  for (let i = 0; i < split.length; i++) {
    let buffer = split[i];
    if (specials.filter(operator => operator.startsWith(buffer)).length > 0) {
      while (specials.filter(operator => operator.startsWith(buffer + split[i + 1])).length > 0) {
        i++;
        buffer += split[i];
      }
      tokens.push({ token: 'special', value: buffer });
    } else if (refStarter.test(buffer || '')) {
      while (refContinuer.test(split[i + 1] || '')) {
        i++;
        buffer += split[i];
      }
      if (builtinValues[buffer] !== undefined) {
        tokens.push({ token: 'value', value: builtinValues[buffer] });
      } else {
        tokens.push({ token: 'ref', value: buffer });
      }
    } else if (buffer === '@') {
      tokens.push({ token: 'ref', value: '@' });
    } else if (numStarter.test(buffer || '')) {
      while (numContinuer.test(split[i + 1] || '')) {
        i++;
        buffer += split[i];
      }
      tokens.push({ token: 'value', value: parseFloat(buffer) });
    } else if (buffer === '"') {
      buffer = '';
      while (split[i + 1] !== '"') {
        i++;
        if (split[i] === '\\') {
          i++;
        }
        buffer += split[i];
      }
      tokens.push({ token: 'value', value: buffer });
      i++;
    } else {
      throw new Error("Lexer Error");
    }
  }
  return tokens;
}


type ParseResult = {
  result: ASTExpression,
  remaining: LexToken[]
};

type Parser = (tokens: LexToken[]) => ParseResult

const tmatch = (token: string, value: unknown, root: LexToken) => {
  return root.token === token && root.value === value;
};

const isBinExp = (token: LexToken) => {
  const res = token.token === 'special' && binaryExpressionStrings.indexOf(token.value) > -1;
  return res;
}

const consumeParenthetical: Parser = (tokens: LexToken[]) => {
  console.log('consumeParenthetical');
  console.log(tokens);
  let current = tokens;
  if (!tmatch('special', '(', current[0])) {
    throw new Error("Parenthetical Issue");
  }
  current = current.slice(1);
  const {
    result,
    remaining
  } = consumeExpression(current);
  current = remaining;
  if (!current[0]) {
    throw new Error("Unexpected EOF");
  }
  if (!tmatch('special', ')', current[0])) {
    throw new Error("Expected )");
  }
  current = current.slice(1)
  return {
    result,
    remaining: current,
  };
}

const consumeArray: Parser = (tokens) => {
  if (!tmatch('special', '[', tokens[0])) {
    throw new Error("ParenStart Issue");
  }
  let current = tokens.slice(1);
  let entries: ASTExpression[] = [];
  while (true) {
    const {
      result,
      remaining
    } = consumeExpression(current);
    entries.push(result);
    current = remaining;
    if (tmatch('special', ',', current[0])) {
      current = current.slice(1);
      continue;
    } else if (tmatch('special', ']', current[0])) {
      current = current.slice(1);
      break;
    } else {
      throw new Error("Unexpected Token " + current[0].value);
    }
  }
  return {
    result: {
      type: 'literal',
      valueType: "array",
      value: entries
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
  for (let i = 0; i < binaryExpressionStrings.length; i++) {
    const currentExpression = binaryExpressionStrings[i];
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
  }

  const joinerPushGuard = (token: LexToken) => {
    if (joiners.length + 1 !== items.length) {
      // Now parsing an item, so guard
      throw new Error("Unexpected Token " + token.value);
    }
  }
  while (current.length > 0) {
    const next = current[0];
    if (isBinExp(next)) {
      joinerPushGuard(next);
      joiners.push(next);
    } else if (tmatch('special', '(', next)) {
      itemPushGuard(next);
      const { result, remaining } = consumeParenthetical(current);
      items.push(result);
      current = remaining;
    } else if (tmatch('special', '[', next)) {
      itemPushGuard(next);
      const { result, remaining } = consumeArray(current);
      items.push(result);
      current = remaining;
    } else if (next.token === "value") {
      itemPushGuard(next);
      items.push({
        type: 'literal',
        valueType: next.value !== null ? typeof next.value as any : 'null',
        value: next.value,
      });
      current = current.slice(1);
    } else if (next.token === 'ref') {
      itemPushGuard(next);
      items.push({
        type: 'reference',
        ref: next.value,
      })
      current = current.slice(1);
    } else {
      break;
      // An unexpected token! Stop parsing this expression
    }
  }

  return {
    result: turnBinaryExpressionSequenceIntoASTExpression({
      items,
      // We know the below is a string because we only add specials
      joiners: joiners.map(joiner => joiner.value as string),
    }),
    remaining: current.slice(),
  }
}

function parseQuery(tokens: LexToken[]): ASTExpression {
  const { result, remaining } = consumeExpression(tokens);
  if (remaining.length !== 0) {
    throw new Error("Unexpected token " + remaining[0].value);
  }
  return result
}

function parse(raw: string): ASTExpression {
  const lexed = lexer(raw);
  const parsed = parseQuery(lexed);
  return parsed;
}

export const parseOrThrow = parse;