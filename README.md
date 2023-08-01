# Filedrop

Filedrop is a minimal, CLI-friendly file transfer service. It is designed to
facilitate quick file uploads/downloads from the terminal.

## Security

This is alpha software at best. It was written in about 30 minutes. There are
ZERO security checks. It is your responsibility to sandbox Filedrop and/or
secure the environment it runs in. At a minumum, containerizing Filedrop with
Docker using the provided Dockerfile is recommended.

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
curl http://filedrop.example.org/5WyQxQu6XAoh >example.txt
```

> The URL shown in the `curl` command can also be accessed via the browser.

## Deployment

To run Filedrop during development, the following should be sufficient:

```sh
poetry install
poetry run uvicorn filedrop:app
```

Alternatively, you may build and run the Docker image.

## License

Copyright &copy; 2022&ndash;2023 Jon Palmisciano. All rights reserved.

Use of this source code is governed by the BSD 3-Clause license; a full copy of
the license can be found in the [LICENSE.txt](LICENSE.txt) file.
