from hashlib import sha256


def hashfile(filename: str, chunk_size: int = 4096) -> str:
    hash = sha256()

    with open(filename, mode="rb") as f:
        for chunk in iter(lambda: f.read(chunk_size), b""):
            hash.update(chunk)

    return hash.hexdigest()
