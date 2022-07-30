import re


def core(x: str):
    return eval(x)


def test(x: str):
    cache = [[], 0]
    for i in x.split("="):
        if re.search("[a-zA-Z]+", i) is None:
            cache[1] = core(i)
        else:
            cache[0] = list(
                filter(lambda x: x != '', re.split("([a-zA-Z]+)", i)))
    return "x="+str(core(f"{cache[1]}/{cache[0][0]}"))


print(test("2x=4+4"))
