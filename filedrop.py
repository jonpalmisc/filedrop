# ===-- Filedrop ----------------------------------------------------------=== #
#
# Copyright (c) 2022-2023 Jon Palmisciano
#
# Redistribution and use in source and binary forms, with or without
# modification, are permitted provided that the following conditions are met:
#
# 1. Redistributions of source code must retain the above copyright notice,
#    this list of conditions and the following disclaimer.
#
# 2. Redistributions in binary form must reproduce the above copyright notice,
#    this list of conditions and the following disclaimer in the documentation
#    and/or other materials provided with the distribution.
#
# 3. Neither the name of the copyright holder nor the names of its contributors
#    may be used to endorse or promote products derived from this software
#    without specific prior written permission.
#
# THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
# AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
# IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
# ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
# LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
# CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
# SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
# INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
# CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
# ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
# POSSIBILITY OF SUCH DAMAGE.
#
# ===----------------------------------------------------------------------=== #

from typing import Optional

from fastapi import FastAPI, Request, Response
from fastapi.responses import PlainTextResponse
from pydantic_settings import BaseSettings, SettingsConfigDict
import shortuuid


class Settings(BaseSettings):
    model_config = SettingsConfigDict(env_prefix="filedrop_")

    host: str = "localhost"
    port: Optional[int] = 3000
    depot: str = "depot"

    allow_upload: bool = True
    allow_download: bool = True


settings = Settings()
app = FastAPI()


def public_display_host() -> str:
    """Get the formatted public host string."""

    display_host = f"http://{settings.host}"
    if (port := settings.port) and port != 80:
        display_host += f":{settings.port}"

    return display_host


def local_depot_path(id: str) -> str:
    """Get the local path for an uploaded/uploading file given its UUID."""

    return f"{settings.depot}/{id}"


def depot_download_url(id: str) -> str:
    """Get the public-facing download URL for a depot file."""

    return f"{public_display_host()}/{id}"


@app.get("/")
async def root():
    return PlainTextResponse(
        f"Use `curl -T <file> {public_display_host()}` to upload files.\n"
    )


@app.put("/{name}")
async def upload(name: str, request: Request):
    if not settings.allow_upload:
        return PlainTextResponse("New file uploads are prohibited.\n", status_code=403)

    data = await request.body()

    uuid = shortuuid.uuid()[:12]
    with open(local_depot_path(uuid), "wb") as f:
        f.write(data)

    if settings.allow_download:
        return PlainTextResponse(f"curl {depot_download_url(uuid)} >{name}\n")
    else:
        return PlainTextResponse(f"{uuid}\n")


@app.get("/{token}")
async def download(token: str):
    try:
        # Shortcut to prohibit downloading when disabled in settings, while
        # also re-using the same error message as to not indicate file
        # existence nor settings state.
        if not settings.allow_download:
            raise FileNotFoundError()

        with open(local_depot_path(token), "rb") as f:
            return Response(f.read())

    except FileNotFoundError:
        return PlainTextResponse("File not found.\n", status_code=404)
