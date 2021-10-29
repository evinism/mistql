import { binaryExpressionStrings, builtinValues, specials } from "./constants";
import { LexError } from "./errors";
import { LexToken } from "./types";

// This defines how various tokens capture whitespace
const whitespaceBehavior = {
  l: ")}]".split(""),
  r: "({[".split(""),
  rl: ".:|,".split("").concat(binaryExpressionStrings),
};

// gross hacky way to ensure that we can tell between string literal end and escaped quote;
function endsWithOddNumberOfSlashes(str: string) {
  return (str.match(/\\+$/) || [""])[0].length % 2 === 1;
}

// TODO: Make this not n squared
const vaccumsWhitespace = (token: string, direction: "l" | "r") => {
  if (whitespaceBehavior.rl.indexOf(token) > -1) {
    return true;
  }
  if (whitespaceBehavior[direction].indexOf(token) > -1) {
    return true;
  }
  return false;
};
const refStarter = /[a-zA-Z_]/;
const refContinuer = /[a-zA-Z_0-9]/;
const numStarter = /[0-9]/;
const numContinuer = /[0-9\.]/;
const whitespace = /\s/;

export function lex(raw: string): LexToken[] {
  const tokens: LexToken[] = [];
  const split = raw.split("");
  for (let i = 0; i < split.length; i++) {
    // For use in position field in tokens
    const position = i;
    let buffer = split[i];
    if (numStarter.test(buffer || "")) {
      while (numContinuer.test(split[i + 1] || "")) {
        i++;
        buffer += split[i];
      }
      tokens.push({
        token: "value",
        value: parseFloat(buffer),
        position,
      });
    } else if (whitespace.test(buffer || "")) {
      while (whitespace.test(split[i + 1])) {
        i++;
      }
      tokens.push({
        token: "special",
        value: " ",
        position,
      });
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
      if (vaccumsWhitespace(buffer, "r")) {
        while (whitespace.test(split[i + 1])) {
          i++;
        }
      }
      if (
        vaccumsWhitespace(buffer, "l") &&
        tokens.length > 0 &&
        tokens[tokens.length - 1].token === "special" &&
        tokens[tokens.length - 1].value === " "
      ) {
        tokens.pop();
      }
      tokens.push({
        token: "special",
        value: buffer,
        position,
      });
    } else if (refStarter.test(buffer || "")) {
      while (refContinuer.test(split[i + 1] || "")) {
        i++;
        buffer += split[i];
      }
      if (builtinValues[buffer] !== undefined) {
        tokens.push({
          token: "value",
          value: builtinValues[buffer],
          position,
        });
      } else {
        tokens.push({
          token: "ref",
          value: buffer,
          position,
        });
      }
    } else if (buffer === "@" || buffer === "$") {
      tokens.push({
        token: "ref",
        value: buffer,
        position,
      });
    } else if (buffer === '"') {
      buffer = "";
      while (split[i + 1] !== '"') {
        i++;
        if (split[i] === undefined) {
          throw new LexError("Unterminated string literal", position, raw);
        } else if (
          split[i] === "\\" &&
          split[i + 1] === '"' &&
          !endsWithOddNumberOfSlashes(buffer)
        ) {
          // Handle escaped double quotes separately
          // The only case where this isn't valid is when the slash before is escaped.
          // Checking for buffer ending with even number of slashes is a hacky way to do this.
          buffer += "\\u0022";
          i++;
        } else {
          buffer += split[i];
        }
      }
      let value: string = "";
      try {
        value = JSON.parse(`"${buffer}"`);
      } catch (e) {
        throw new LexError("Invalid string literal", position, raw);
      }
      tokens.push({
        token: "value",
        value,
        position,
      });
      i++;
    } else {
      throw new LexError(`Unexpected character '${split[i]}'`, i, raw);
    }
  }

  // Trim whitespace as a post-lex step
  const firstToken = tokens[0];
  if (
    firstToken &&
    firstToken.token === "special" &&
    firstToken.value === " "
  ) {
    tokens.shift();
  }

  const lastToken = tokens[tokens.length - 1];
  if (lastToken && lastToken.token === "special" && lastToken.value === " ") {
    tokens.pop();
  }

  return tokens;
}
