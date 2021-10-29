

export const makeIndicator = (text: string, position: number) => {
  const prevStringLines = text.substring(0, position).replace(/[^\s]/g, ' ').split('\n');
  const lines = text.split('\n');
  const indicatorLine = prevStringLines.length - 1;
  const indicator = prevStringLines.slice(-1)[0] + '^';
  const formatted = [].concat(
    lines.slice(0, indicatorLine + 1),
    [indicator],
    lines.slice(indicatorLine + 1)
  ).join('\n');
  return formatted;
}

const makeMessage = (message: string, position: number, query: string) =>
  `${message} at position ${position}
---
${makeIndicator(query, position)}
---
`;

export class PositionableError extends Error {
  constructor(message: string, position: number, query: string) {
    super(makeMessage(message, position, query));
  }
}

export class LexError extends PositionableError {
}

export class ParseError extends PositionableError {
}

export class UnpositionableParseError extends Error {
}

export class RuntimeError extends Error {
}

export class OpenAnIssueIfThisOccursError extends Error { }