# Filedrop

Filedrop is a minimal, CLI-friendly file transfer service. It is designed to
facilitate quick file uploads/downloads from the terminal.

## Security

This is alpha software at best. It was written in a few hours. There are ZERO
security checks. It is your responsibility to secure the environment Filedrop
runs in. At a minumum, containerizing Filedrop with Docker using the provided
Dockerfile is recommended.

Do **NOT** use this in production yet. Seriously.

## Usage

Filedrop does not have a concept of accounts or users. Files are simply uploaded
using `curl`:

```sh
curl -T example.txt filedrop.example.org
```

Upon successfully uploading a file, the server should respond with the `curl`
command needed to retrieve the file; you can send this to the intended
recipient. A sample response is shown below.

```
curl -O http://filedrop.example.org/get/EAE96A46968A47CF813963E42FE3C146-example.txt
```

> The URL shown in the `curl` command can also be accessed via the browser.

As seen above, random prefixes are prepended to uploaded file names to prevent
naming conflicts and the ability to guess filenames.

## Deployment

Use `cargo` to build Filedrop, then run the resulting `filedrop` executable to
start the server. Filedrop can be configured through a handful of environment
variables; run `filedrop -h` for more information.

For convenience, Filedrop can also be built as a Docker image and containerized
using the provided Dockerfile.

## License

Copyright &copy; 2022 Jon Palmisciano. All rights reserved.

Use of this source code is governed by the BSD 3-Clause license; a full copy of
the license can be found in the [LICENSE.txt](LICENSE.txt) file.
