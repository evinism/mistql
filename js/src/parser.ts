import { isLeft } from "fp-ts/lib/Either";
import { either, map, Parser } from "parser-ts/lib/Parser";
import { stream } from "parser-ts/lib/Stream";
import { doubleQuotedString, float as floatParser, string as stringParser } from "parser-ts/lib/string";
import { ASTExpression } from "./types";

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

const trueLiteralParser = map(() => true)(stringParser('true'));
const falseLiteralParser = map(() => false)(stringParser('false'));
const booleanParser = map((bool: boolean) => ({
  type: "literal" as "literal",
  valueType: "boolean" as "boolean",
  value: bool
}))(either(trueLiteralParser, () => falseLiteralParser));


const listOfExpressionParsers: Parser<string, ASTExpression>[] = [
  doubleQuotedLiteralParser,
  nullLiteralParser,
  numberLiteralParser,
  booleanParser
];

const expressionParser: Parser<string, ASTExpression> =
  listOfExpressionParsers.reduce((acc, cur) => either(acc, () => cur));

export const parse = (raw: string) => expressionParser(stream(raw.split(''), 0));


export const parseOrThrow = (raw: string): ASTExpression => {
  const result = parse(raw);
  if (isLeft(result)) {
    throw new Error(`Expected one of: ${result.left.expected}`);
  } else {
    return result.right.value;
  }
}
