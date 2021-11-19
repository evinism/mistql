from lark import Tree


def execute(ast: Tree, data):
    # This is all wrong but damn 
    stack = [
        {
            "@": data,
            "null": None,
            "true": True,
            "false": False,
        }
    ]
    if ast.data == "namedref":
        return stack[-1][ast.children[0].value]
    elif ast.data == "at":
        return stack[-1]["@"]
    elif ast.data == "dollar":
        return stack[-1]["$"]
    if ast.data == "number":
        return float(ast.children[0].value)
    elif ast.data == "string":
        return ast.children[0].value
    elif ast.data == "array":
        return [execute(child, stack) for child in ast.children]
    elif ast.data == "true":
        return True
    elif ast.data == "false":
        return False
    elif ast.data == "null":
        return None


    raise NotImplementedError("execute() not implemented for " + ast.data)
