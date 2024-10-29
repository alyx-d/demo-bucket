from random import random

import orjson

import hello
import util


def main():
    print(hello.name)
    print(hello.age)
    print(util.lang())
    print(util.add(1, 2, 3, 4, 5))


def json_test():
    json_str = [
        {
            "id": i,
            "value": str(i) + "hello"
        }
        for i in range(100_0000)
    ]
    s = orjson.dumps(json_str)


if __name__ == "__main__":
    main()
