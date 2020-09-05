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

*The data received could be any set of bytes; the data sent back is always UTF-8
encoded.*

Any data sent in the body of any request is parsed through a number of encoding 
engines. Each engine either returns an error because the data could not be
parsed in its encoding, or the reencoded string followed by a character by
character description of the string.

Currently implemented engines:
* UTF-8

## Example

```bash
$ xxd -p t
61c39fc999
$ cat t | curl localhost -d @-
aßə
a  U+0061 LATIN SMALL LETTER A
ß  U+00DF LATIN SMALL LETTER SHARP S
ə  U+0259 LATIN SMALL LETTER SCHWA
```
