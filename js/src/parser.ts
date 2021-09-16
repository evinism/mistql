import { isLeft } from "fp-ts/lib/Either";
import { alphanum, char } from "parser-ts/lib/char";
import { apFirst, either, eof, many1, map, Parser, sepBy, sepBy1 } from "parser-ts/lib/Parser";
import { stream } from "parser-ts/lib/Stream";
import { doubleQuotedString, float as floatParser, string as stringParser } from "parser-ts/lib/string";
import { ASTExpression, ASTPipelineExpression } from "./types";

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

const reference = map((raw: string[][]) => {
  const path = raw.map(nameArr => nameArr.join(''));
  return {
    type: "reference" as "reference",
    path: path,
  }
})(sepBy(char('.'), either(many1(alphanum), () => map(() => ["@"])(char("@")))))

const listOfNonPipelineExpressionParsers: Parser<string, ASTExpression>[] = [
  doubleQuotedLiteralParser,
  nullLiteralParser,
  numberLiteralParser,
  booleanParser,
  reference,
];

const nonPipelineExpressionParser: Parser<string, ASTExpression> =
  listOfNonPipelineExpressionParsers.reduce((acc, cur) => either(acc, () => cur));

const pipelineParser = map((stages: ASTExpression[]) => ({
  type: "pipeline" as "pipeline",
  stages,
}))(sepBy1(char('|'), nonPipelineExpressionParser));


const expressionParser: Parser<string, ASTExpression> = map((node: ASTPipelineExpression) => {
  if (node.stages.length === 1) {
    return node.stages[0];
  } else {
    return node;
  }
})(pipelineParser);

const statementParser = apFirst<string, void>(eof())(expressionParser)

// Normalizes and removes unnecessary whitespace
const whiteSpacealyzer = (str: string) => {
  // TODO: Make these more solid.
  return str
    .replace(/\s+/g, ' ')
    .replace(/\s*\.\s*/g, '.')
    .replace(/\s*\(\s*/g, '(')
    .replace(/\s*\)\s*/g, ')')
    .replace(/\s*\|\s*/g, '|')
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
