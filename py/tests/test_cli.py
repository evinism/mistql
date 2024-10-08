import tempfile
from mistql.cli import main


nice_string = '{"hello": "there"}'


naughty_strings = [
    '{"key": ",。・:*:・゜’( ☻ ω ☻ )。・:*:・゜’"}',
    '{"key": "👾 🙇 💁 🙅 🙆 🙋 🙎 🙍"}',
    '{"key": "👨‍👩‍👦 👨‍👩‍👧‍👦 👨‍👨‍👦 👩‍👩‍👧 👨‍👦 👨‍👧‍👦 👩‍👦 👩‍👧‍👦"}',
    '{"key": "̡͓̞ͅI̗̘̦͝n͇͇͙v̮̫ok̲̫̙͈i̖͙̭̹̠̞n̡̻̮̣̺g̲͈͙̭͙̬͎ ̰t͔̦h̞̲e̢̤ ͍̬̲͖f̴̘͕̣è͖ẹ̥̩l͖͔͚i͓͚̦͠n͖͍̗͓̳̮g͍ ̨o͚̪͡f̘̣̬ ̖̘͖̟͙̮c҉͔̫͖͓͇͖ͅh̵̤̣͚͔á̗̼͕ͅo̼̣̥s̱͈̺̖̦̻͢.̛̖̞̠̫̰"}',  # noqa: E501
]


all_strings = [nice_string] + naughty_strings


def enc_helper(encoding, string):
    input_file = tempfile.NamedTemporaryFile(delete=False)
    input_file.write(string.encode(encoding))
    input_file.close()
    output_file = tempfile.NamedTemporaryFile(delete=False)
    output_file.close()
    main(["@", "--file", input_file.name, "--output", output_file.name])
    with open(output_file.name, "rb") as f:
        assert f.read().decode("utf-8") == string


def test_encoding_utf8():
    for string in all_strings:
        enc_helper("utf-8", string)


def test_encoding_utf16():
    for string in all_strings:
        enc_helper("utf-16", string)


def test_encoding_utf32():
    for string in all_strings:
        enc_helper("utf-32", string)


def test_encoding_ascii():
    enc_helper("ascii", nice_string)
