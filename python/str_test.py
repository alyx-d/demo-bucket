from collections import defaultdict


def main():
    s = {1, 2, 3, 3}
    print(type(s), s)


def log(*args, **kwargs):
    def wapper(func):
        def invoke():
            print(args, kwargs)
            print("before")
            func()
            print("after")

        return invoke

    return wapper


@log(1, 2, 3, 4, 5, 5, a=1, b=2)
def main():
    print("main func")


def testWith():
    with open("read.py") as file:
        for line in file.readlines():
            print(line, end="")


if __name__ == "__main__":
    testWith()
