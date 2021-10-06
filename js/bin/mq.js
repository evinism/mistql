#!/usr/bin/env node
const fs = require("fs");
const mistql = require("../dist/umd/index");

function helpAndExit() {
  console.log(`Usage: ${argv[0]} <query> [file]`);
  process.exit(0);
}

function errorAndExit(msg) {
  console.error(msg);
  process.exit(1);
}

let argv = process.argv.slice(1);
const flags = [];

const shorthands = {
  h: "help",
};

{
  const next = [];
  for (let i = 0; i < argv.length; i++) {
    const curr = argv[i];
    if (curr.startsWith("--")) {
      flags.push(curr.substring(2));
    } else if (curr.startsWith("-")) {
      curr
        .substring(1)
        .split("")
        .forEach((item) => {
          const longflag = shorthands[item];
          if (!longflag) {
            errorAndExit("Unknown flag " + item);
          }
          flags.push(longflag);
        });
    } else {
      next.push(curr);
    }
  }
  argv = next;
}

for (let i = 0; i < flags.length; i++) {
  if (flags[i] === "help") {
    helpAndExit();
  } else {
    errorAndExit("Unknown flag " + flags[i]);
  }
}

if (argv.length < 2) {
  helpAndExit();
}

if (argv.length > 3) {
  helpAndExit();
}

const [, query, file] = argv;
const data = JSON.parse(fs.readFileSync(file || 0).toString());

try {
  console.log(JSON.stringify(mistql.query(query, data), null, 2));
} catch (e) {
  console.error(e.message);
}
