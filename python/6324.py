from urllib.parse import urlparse

for i in range(int(input())):
    # fmt: off
    protocol, hostname, path, _, query, fragment = urlparse(url := input())
    # fmt: on
    host = (hostname := hostname.split(":"))[0]
    port = hostname[1] if len(hostname) > 1 else ""
    url = url[url.index("://") + len("://") :]
    path = (url[url.index("/") + 1 :] if url.find("/") else "") if query or fragment else path[1:]

    print(f"URL #{i+1}")
    print(f"Protocol = {protocol}")
    print(f"Host     = {host}")
    print(f"Port     = {port if port else '<default>'}")
    print(f"Path     = {path if path else '<default>'}")
    print("")
