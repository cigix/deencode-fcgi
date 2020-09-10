# `deencode_fcgi`: A FastCGI character encoding analyzer

## How to use

1. Download the latest Unicode Database:
   ```bash
   wget https://www.unicode.org/Public/UCD/latest/ucd/UnicodeData.txt
   ```
2. Start the `deencode_fcgi` server:
   ```bash
   cargo run
   ```
3. Start your FastCGI enabled web server. The FastCGI server listens on
   `localhost:9000`. An example server configuration for NGINX is given in
   [`deencode.nginx.example`][deencode.nginx.example].
4. Shazam! You should now have an endpoint of your web server connected to
   `deencode_fcgi`.

## Behaviour

*The data received could be any set of bytes; `deencode_fcgi` ignores the HTTP
header completely, including the request method and `Content-Type` parameter.*

*The data sent back by `deencode_fcgi` is always valid JSON, with all non-ASCII
characters escaped.*

Any data sent in the body of any request is parsed through a number of encoding 
engines. Each engine will give back an object with:
* in case of parsing error:
  * an "error" field, a string with the reason for the parsing error
* in case of successful parsing:
  * a "parsed" field, a string as parsed by the engine"
  * a "description" field, an array of objects for each character of the form:
    * "character", a string containing only the character
    * "codepoint", a string with the usual text representation of that character
      for that encoding
    * "name", a string with a description of the character

Currently implemented engines:
* UTF-8
* UTF-16 Big endian
* UTF-16 Little endian

## Example

```bash
$ xxd -p t
61c39fc999
$ curl localhost --data-binary @t | python3 -m json.tool | xargs -0 printf
{
    "UTF-8": {
        "description": [
            {
                "character": "a",
                "codepoint": "U+0061",
                "name": "LATIN SMALL LETTER A"
            },
            {
                "character": "ß",
                "codepoint": "U+00DF",
                "name": "LATIN SMALL LETTER SHARP S"
            },
            {
                "character": "ə",
                "codepoint": "U+0259",
                "name": "LATIN SMALL LETTER SCHWA"
            }
        ],
        "parsed": "aßə"
    }
}
```
