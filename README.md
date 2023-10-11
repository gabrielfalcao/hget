# HGET

Simple Terminal Utility: Resolves Hostnames

```bash
$ hget --help
Resolves Hostnames

Usage: hget [OPTIONS] <HOSTNAMES>...

Arguments:
  <HOSTNAMES>...

Options:
  -p, --port <PORT>  [default: 80]
  -c, --crash        crash upon non-ability to name-resolution
  -s, --show-domain  respectively displays the domain names wherein IP-address resolve
  -h, --help         Print help
  -V, --version      Print version
```


## Examples


```bash
hget -s apple.com alphabet.xyz amazon.com meta.com
```
