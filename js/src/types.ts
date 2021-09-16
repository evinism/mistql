// Parser Types 
export type ASTLiteralExpression = {
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
  value: Array<ASTExpression>,
} | {
  type: "literal",
  valueType: 'null',
  value: null
}

export type ASTPipelineExpression = {
  type: "pipeline"
  stages: ASTExpression[],
}

export type ASTReferenceExpression = {
  type: 'reference',
  path: string[],
}

export type ASTApplicationExpression = {
  type: "application"
  function: ASTExpression,
  arguments: ASTExpression[]
}

export type ASTExpression = ASTApplicationExpression | ASTReferenceExpression| ASTPipelineExpression | ASTLiteralExpression;

