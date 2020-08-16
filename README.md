A tiny (VeRy sEnSiBlE) Rust library and CLI app for creating [Mocking SpongeBob](https://knowyourmeme.com/memes/mocking-spongebob) text from normal input.

## Usage
### CLI

Simply provide the input to be MoCkEd as arguments to the app. Each input argument will be output on its own line.

```
$ spongemock "this is a" "worthwhile use of my time"
ThIs Is A
WoRtHwHiLe UsE oF mY tImE
```

If no arguments are given, input will be read from stdin.

```
$ fortune | spongemock | cowsay
 ________________________________________
/ YoU wIlL iNhErIt SoMe MoNeY oR a SmAlL \
\ pIeCe Of LaNd.                         /
 ----------------------------------------
        \   ^__^
         \  (oo)\_______
            (__)\       )\/\
                ||----w |
                ||     ||
```

### Library

See the doc comments in [lib.rs](src/lib.rs).