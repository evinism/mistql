import { isLeft } from "fp-ts/lib/Either";
import { alphanum, char } from "parser-ts/lib/char";
import { apFirst, between, either, eof, many1, map, Parser, sepBy, sepBy1, seq, succeed } from "parser-ts/lib/Parser";
import { stream } from "parser-ts/lib/Stream";
import { doubleQuotedString, float as floatParser, string as stringParser } from "parser-ts/lib/string";
import { ASTExpression, ASTPipelineExpression } from "./types";

// To help with circular refs. 
const lazyRef: <A, B>(fn: () => Parser<A, B>) => Parser<A, B> = (fn) => seq(succeed(undefined), fn);
const lazySimpleExpressionParser = lazyRef(() => simpleExpressionParser);
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

// This might be order of precedence, but it needs testing
const binaryExpressionStrings = [
  '+',
  '-',
  '*',
  '/',
  '&&',
  '||',
  '==',
  '!=',
  '<',
  '>',
  '<=',
  '>=',
];

const matchBinExpParser = binaryExpressionStrings.map(stringParser).reduce((acc, cur) => either(acc, () => cur));

const binaryExpressionParser = seq(
  lazySimpleExpressionParser, 
  (first) => seq(
    lazySimpleExpressionParser, 
    (second) => map((operator: string) => ({
      type: 'application' as 'application',
      function: {
        type: "reference" as 'reference',
        path: [operator]
      },
      arguments: [first, second],
    }))(matchBinExpParser)))

const listOfSimpleExpressionParsers: Parser<string, ASTExpression>[] = [
  parentheticalExpression,
  arrayLiteralParser,
  doubleQuotedLiteralParser,
  nullLiteralParser,
  numberLiteralParser,
  booleanParser,
  referenceParser,
  binaryExpressionParser,
];

const simpleExpressionParser: Parser<string, ASTExpression> =
  listOfSimpleExpressionParsers.reduce((acc, cur) => either(acc, () => cur));

const compoundExpressionParser: Parser<string, ASTExpression> = map((simpleExpressions: ASTExpression[]) => {
  if (simpleExpressions.length === 1) {
    return simpleExpressions[0]
  }
  return {
    type: "application" as "application",
    function: simpleExpressions[0],
    arguments: simpleExpressions.slice(1),
  }
})(sepBy1(char(' '), simpleExpressionParser))

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
  return str
    .trim()
    .replace(/\s+/g, ' ')
    .replace(/\s*\.\s*/g, '.')
    .replace(/\(\s*/g, '(')
    .replace(/\s*\)/g, ')')
    .replace(/\s*\|\s*/g, '|')
    .replace(/\s*,\s*/g, ',')
    .replace(/\s*==\s*/g, '==')
    .replace(/\s*&&\s*/g, '&&')
    .replace(/\s*\|\|\s*/g, '||');
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
