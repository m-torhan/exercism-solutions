def rebase(input_base: int, digits: list, output_base: int) -> list:
    if input_base < 2:
        raise ValueError("input base must be >= 2")
    if output_base < 2:
        raise ValueError("output base must be >= 2")
    for d in digits:
        if d < 0 or d >= input_base:
            raise ValueError("all digits must satisfy 0 <= d < input base")

    num = sum(d * input_base**i for i, d in enumerate(digits[::-1]))

    ret = []

    while num > 0:
        ret.insert(0, num % output_base)
        num //= output_base

    if not len(ret):
        ret.append(0)

    return ret
