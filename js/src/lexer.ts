import { binaryExpressionStrings, builtinValues, specials } from "./constants";
import { LexError } from "./errors";
import { LexToken } from "./types";
import { escapeRegExp } from "./util";

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

const refStarter = /[a-zA-Z_]/;
const refContinuer = /[a-zA-Z_0-9]/;
const numStarter = /[0-9]/;
const numContinuer = /[0-9\.]/;

export function lex(raw: string): LexToken[] {
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
        if (split[i] === undefined) {
          throw new LexError("Unterminated string literal");
        }
        buffer += split[i];
      }
      tokens.push({ token: "value", value: buffer });
      i++;
    } else {
      throw new LexError("Unexpected token " + split[i]);
    }
  }
  return tokens;
}
