A tiny (VeRy sEnSiBlE) Rust library and CLI app for creating [Mocking SpongeBob](https://knowyourmeme.com/memes/mocking-spongebob) text from normal input.

## Usage
### CLI

Simply provide the input to be MoCkEd as arguments to the app. Each input argument will be output on its own line.

```
$ spongemock "this is a" "worthwhile use of my time"
ThIs iS A
WoRtHwHiLe uSe oF My tImE
```

If no arguments are given, input will be read from stdin.

```
$ fortune | spongemock | cowsay
 ________________________________________
/ YoU WiLl iNhErIt sOmE MoNeY Or a sMaLl \
\ pIeCe oF LaNd.                         /
 ----------------------------------------
        \   ^__^
         \  (oo)\_______
            (__)\       )\/\
                ||----w |
                ||     ||
```

### Library

See the doc comments in [lib.rs](src/lib.rs).