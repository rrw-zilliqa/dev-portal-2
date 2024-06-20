# Zilliqa developer documentation

This repository contains the Zilliqa developer documentation.

- The `main` branch has CD at [https://developer-portal.zilstg.dev/]
- Releases to go to [https://developer-portal.zilliqa.com]

## How it works

The developer portal is a pair of `mkdocs` materials-themed sites -
one for Zilliqa 1 and one for Zilliqa 2.

The Zilliqa 1 site is static.
The Zilliqa 2 site includes content created by the rust program in `docgen` - see below.

They are served in production by an nginx container containing the
routing between the two, built by the `Dockerfile` and using the
config file from `default.conf`.

The material theme is fairly heavily customised with overrides in both
Zilliqa 1 and Zilliqa 2.

## Docgen

The `docgen` program:

- Reads the `zq2_spec.yaml` file in this directory.
- Clones Zilliqa 2 into `cache/zq2`
- Runs the `docgen` program in Zilliqa 2 to generate API documentation.
- Repeats the process for all the versions of Zilliqa 2 it is to document.
- Generates an `mkdocs.yaml`

`mkdocs.yaml` for zq2 is generated from:

- `zq2/mkdocs.in.yaml`, which inherits from
- `zq2/parenty.yaml`

## Developing

Development is controlled by the `Makefile` in this directory.

You will need `mkdocs`. Make sure you're in a venv (because `mkdocs`
doesn't seem to like installing outside one):

```sh
python -m venv ~/mydir
source ~/mydir/bin/activate
```

And then:

```sh
pip3 install -r requirements.txt
```

You will also need enough local tooling installed to build ZQ2; this
starts with [installing
Rust](https://www.rust-lang.org/tools/install), but there are other
tools - including cmake and protobuf required. Check the [zq2
repository](https://github.com/zilliqa/zq2) for details.

Now,

- `make dev1` will make and serve the ZQ1 docs locally on port 8000.
- `make dev2` will do the same with the ZQ2 docs.

You can set `SERVEROPTS` to pass option (usually `-a
<listen_address>:<port>`) to `mkdocs serve`.

If you want to check containerised builds, `make run-image` will do
that for you and run it on port 8080.
