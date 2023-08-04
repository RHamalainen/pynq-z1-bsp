if __name__ == "__main__":
    from pathlib import Path

    path_examples = Path.cwd() / Path("examples/")
    if not path_examples.exists():
        print(f"Path to examples does not exist.")
        print(f" - {path_examples}")
        exit(1)
    if not path_examples.is_dir():
        print(f"Path to examples is not a directory.")
        print(f" - {path_examples}")
        exit(1)

    for (index, path) in enumerate(path_examples.iterdir()):
        print(f"{index + 1} {path.stem}")
