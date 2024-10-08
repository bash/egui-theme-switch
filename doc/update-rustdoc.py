#!/usr/bin/env python3

from base64 import b64encode


def update_rustdoc():
    with open("doc/preview.md", "w+") as fp:
        fp.write(
            template(
                dark=base64_encode("doc/dark.png"),
                light=base64_encode("doc/light.png"),
            )
        )


def base64_encode(file_path):
    with open(file_path, "rb") as fp:
        return b64encode(fp.read()).decode("utf-8")


def template(*, dark, light):
    return f"""<!-- This file is auto-generated by dpc/update_rustdoc.py -->
<picture>
    <source media="(prefers-color-scheme: dark)" srcset="data:image/svg+xml;base64,{dark}">
    <img src="data:image/svg+xml;base64,{light}" width="400">
</picture>
"""


if __name__ == "__main__":
    update_rustdoc()
