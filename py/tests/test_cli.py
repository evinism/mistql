import tempfile
from mistql.cli import main


nice_string = '{"hello": "there"}'


naughty_string = '{"hello": true,"unicode": ",。・:*:・゜’( ☻ ω ☻ )。・:*:・゜’",' + \
                 '"emojis":"👾 🙇 💁 🙅 🙆 🙋 🙎 🙍"}'


def enc_helper(encoding, string):
    input_file = tempfile.NamedTemporaryFile()
    input_file.write(string.encode(encoding))
    input_file.flush()
    output_file = tempfile.NamedTemporaryFile()
    main(["@", "--file", input_file.name, "--output", output_file.name])
    assert output_file.read().decode("utf-8") == string


def test_encoding_utf8():
    enc_helper("utf-8", nice_string)
    # enc_helper("utf-8", naughty_string)


def test_encoding_utf16():
    enc_helper("utf-16", nice_string)
    # enc_helper("utf-16", naughty_string)


def test_encoding_ascii():
    enc_helper("ascii", nice_string)
