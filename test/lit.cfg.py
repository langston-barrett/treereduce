# type: ignore

"""Configure `lit`"""

import lit.formats

config.name = "treereduce"  # noqa: F821
config.test_format = lit.formats.ShTest(True)  # noqa: F821
config.suffixes = [".c", ".dl", ".rs"]  # noqa: F821

if __name__ == "__main__":
    from argparse import ArgumentParser

    parser = ArgumentParser(description=__doc__)
    parser.parse()
