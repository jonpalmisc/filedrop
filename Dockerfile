FROM python:3.10-slim

# Files will be always be downloaded to `/depot` inside of the container; this
# path should be mapped to the host using Docker's volume/bind mounts.
ENV FILEDROP_DEPOT="/depot"

WORKDIR /code
COPY filedrop.py poetry.lock pyproject.toml /code

RUN pip install --no-cache-dir poetry
RUN poetry config virtualenvs.create false
RUN poetry --no-root --no-cache --no-interaction install

# The server will always run on port 8080 inside of the container.
#
# Similarly to the depot directory, this should be mapped to the desired public
# port on the host, which should also be specified via the `FILEDROP_PORT`
# environment variable so that returned `curl` invocations are correct.
CMD ["uvicorn", "filedrop:app", "--host", "0.0.0.0", "--port", "8080"]
