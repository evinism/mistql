// General

export type Result<L, R> = {
  success: L,
  error?: void,
} | {
  success?: void
  error: R,
}

// Parser Types 
type ASTLiteral = {
  type: "literal",
  valueType: 'string',
  value: string
} | {
  type: "literal",
  valueType: 'number',
  value: number
} | {
  type: "literal",
  valueType: 'boolean',
  value: boolean
} | {
  type: "literal",
  valueType: 'array',
  value: Array<unknown>,
} | {
  type: "literal",
  valueType: 'null',
  value: null
}

export type ASTPipelineExpression = {
  type: "pipeline"
  stages: ASTExpression[],
}

export type ASTExpression = {
  type: "function"
  function: ASTExpression,
  arguments: ASTExpression[]
} | {
  type: 'reference',
  path: string[],
} | ASTPipelineExpression | ASTLiteral;

export type ParseResult<Err> = Result<ASTExpression, Err>;
