from lark import Lark

mistql_parser = Lark(r"""
?start: expression

?expression: sum | literal

?literal: object
      | array
      | string
      | SIGNED_NUMBER      -> number
      | "true"             -> true
      | "false"            -> false
      | "null"             -> null

array  : "[" [literal ("," literal)*] "]"
object : "{" [pair ("," pair)*] "}"
pair   : string ":" literal
string : ESCAPED_STRING

?sum: product
    | sum "+" product   -> add
    | sum "-" product   -> sub

?product: atom
    | product "*" atom  -> mul
    | product "/" atom  -> div

?atom: NUMBER           -> number
     | "-" atom         -> neg
     | "(" expression ")"
     | literal

%import common.ESCAPED_STRING
%import common.SIGNED_NUMBER
%import common.WS
%import common.NUMBER

%ignore WS


""", parser="earley")

def parse(raw):
    parsed = mistql_parser.parse(raw)
    print(parsed.pretty())
    return parsed
