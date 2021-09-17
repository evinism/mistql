import { isLeft } from "fp-ts/lib/Either";
import { alphanum, char } from "parser-ts/lib/char";
import { apFirst, between, either, eof, many1, many, map, Parser, sepBy, sepBy1, seq, succeed, chain } from "parser-ts/lib/Parser";
import { stream } from "parser-ts/lib/Stream";
import { doubleQuotedString, float as floatParser, string as stringParser } from "parser-ts/lib/string";
import { ASTExpression, ASTPipelineExpression } from "./types";
import { escapeRegExp } from "./util";

// To help with circular refs. 
const lazyRef: <A, B>(fn: () => Parser<A, B>) => Parser<A, B> = (fn) => seq(succeed(undefined), fn);
const lazyExpressionParser = lazyRef(() => expressionParser);


const doubleQuotedLiteralParser = map((str: string) => ({
  type: "literal" as 'literal',
  valueType: "string" as 'string',
  value: str
}))(doubleQuotedString);

const nullLiteralParser = map(() => ({
  type: 'literal' as 'literal',
  valueType: 'null' as 'null',
  value: null,
}))(stringParser('null'));

const numberLiteralParser = map((num: number) => ({
  type: "literal" as "literal",
  valueType: "number" as "number",
  value: num
}))(floatParser);

const arrayLiteralParser: Parser<string, ASTExpression> = map((expressions: ASTExpression[]) => ({
  type: "literal" as "literal",
  valueType: "array" as "array",
  value: expressions,
}))(between(char('['), char(']'))(sepBy(char(","), lazyExpressionParser)))

const trueLiteralParser = map(() => true)(stringParser('true'));
const falseLiteralParser = map(() => false)(stringParser('false'));
const booleanParser = map((bool: boolean) => ({
  type: "literal" as "literal",
  valueType: "boolean" as "boolean",
  value: bool
}))(either(trueLiteralParser, () => falseLiteralParser));

const referenceParser = map((raw: string[][]) => {
  const path = raw.map(nameArr => nameArr.join(''));
  return {
    type: "reference" as "reference",
    path: path,
  }
})(sepBy1(char('.'), either(many1(alphanum), () => map(() => ["@"])(char("@")))))

const parentheticalExpression = between(char('('), char(')'))(lazyExpressionParser);

const listOfSimpleExpressionParsers: Parser<string, ASTExpression>[] = [
  parentheticalExpression,
  arrayLiteralParser,
  doubleQuotedLiteralParser,
  nullLiteralParser,
  numberLiteralParser,
  booleanParser,
  referenceParser,
];

const simpleExpressionParser: Parser<string, ASTExpression> =
  listOfSimpleExpressionParsers.reduce((acc, cur) => either(acc, () => cur));

/* BINARY EXPRESSIONS AKA death by parsing because i don't know what i'm doing */
// This might be order of precedence, but it needs testing
// Through complete luck, we're good with left-to-right associativity.
// This could change with exponentiation
const binaryExpressionStrings = [
  '*',
  '/',
  '%',
  '+',
  '-',
  '<=',
  '>=',
  '<',
  '>',
  '==',
  '!=',
  '&&',
  '||',
];

const matchBinExpParser = binaryExpressionStrings.map(stringParser).reduce((acc, cur) => either(acc, () => cur));

type BinaryExpressionSequence = {items: ASTExpression[], joiners: string[]};
// Parser of type {items: ASTExpression[], joiners: string[]}

const binaryExpressionSequenceParser: Parser<string, BinaryExpressionSequence> = seq(simpleExpressionParser, (head) => map((bexprecords: {
  operator: string,
  rhs: ASTExpression
}[]) => {
  return {
    items: [head].concat(bexprecords.map(r => r.rhs)),
    joiners: bexprecords.map(r => r.operator)
  }
})(many(seq(matchBinExpParser, (operator) => map((secondExp: ASTExpression) => ({
  operator,
  rhs: secondExp
}))(simpleExpressionParser)))));

// This might be the worst function i've ever written.
// But at least it's a contained transformation.
const turnBinaryExpressionSequenceIntoASTExpression = (bexpseq: BinaryExpressionSequence): ASTExpression => {
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
            path: [currentExpression]
          },
          arguments: [l, r]
        }
        newItems.length = newItems.length - 1;
      } else {
        newJoiners.push(current.joiners[j])
      }
    }
    current = {
      items: newItems,
      joiners: newJoiners,
    }
  }
  return current.items[0];
}

const binaryExpressionParser = map(turnBinaryExpressionSequenceIntoASTExpression)(binaryExpressionSequenceParser);
/* END BINARY EXPRESSIONS */


const compoundExpressionParser: Parser<string, ASTExpression> = map((simpleExpressions: ASTExpression[]) => {
  if (simpleExpressions.length === 1) {
    return simpleExpressions[0]
  }
  return {
    type: "application" as "application",
    function: simpleExpressions[0],
    arguments: simpleExpressions.slice(1),
  }
})(sepBy1(char(' '), binaryExpressionParser))

const pipelineExpressionParser = map((stages: ASTExpression[]) => ({
  type: "pipeline" as "pipeline",
  stages,
}))(sepBy1(char('|'), compoundExpressionParser));

const expressionParser: Parser<string, ASTExpression> = map((node: ASTPipelineExpression) => {
  if (node.stages.length === 1) {
    return node.stages[0];
  } else {
    return node;
  }
})(pipelineExpressionParser);

const statementParser = apFirst<string, void>(eof())(expressionParser)

// Normalizes and removes unnecessary whitespace
const whiteSpacealyzer = (str: string) => {
  // TODO: Make these more solid.
  let retval = str
    .trim()
    .replace(/\s+/g, ' ')
    .replace(/\s*\.\s*/g, '.')
    .replace(/\(\s*/g, '(')
    .replace(/\s*\)/g, ')')
    .replace(/\s*\|\s*/g, '|')
    .replace(/\s*,\s*/g, ',')
    binaryExpressionStrings.forEach((binexp) => {
      const re = new RegExp(`\\s*${escapeRegExp(binexp)}\\s*`, 'g');
      retval = retval.replace(re, binexp);
    })
    return retval;
}

export const parse = (raw: string) => statementParser(stream(whiteSpacealyzer(raw).split(''), 0));

export const parseOrThrow = (raw: string): ASTExpression => {
  const result = parse(raw);
  if (isLeft(result)) {
    throw new Error(`Expected one of: ${result.left.expected}`);
  } else {
    return result.right.value;
  }
}
