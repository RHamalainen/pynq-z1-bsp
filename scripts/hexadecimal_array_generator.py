from argparse import ArgumentParser
from dataclasses import dataclass


@dataclass
class ProgramArguments:
    start: int
    final: int
    step: int

    @staticmethod
    def get() -> "ProgramArguments":
        parser = ArgumentParser(
            prog="HexadecimalArrayGenerator",
            description="Generate array of hexadecimal values given start, final and step.",
        )
        parser.add_argument("start")
        parser.add_argument("final")
        parser.add_argument("step")
        arguments = vars(parser.parse_args())
        start = int(arguments["start"], 16)
        final = int(arguments["final"], 16)
        step = int(arguments["step"])
        return ProgramArguments(
            start=start,
            final=final,
            step=step,
        )


if __name__ == "__main__":
    arguments = ProgramArguments.get()
    array_range = range(
        arguments.start, arguments.final + arguments.step, arguments.step
    )
    for i in array_range:
        value = f"{i:08X}"
        left, right = value[0:4], value[4:8]
        value = f"{left}_{right}"
        print(f"0x{value},")
    print(f"{len(array_range)}")
