c = 0

def iota() -> int:
    global c
    c += 1

    return c - 1

def bf(bf_code: str):
    cmd_map = {
        '+': INC,
        '-': DEC,
        '>': SHR,
        '<': SHL,
        '[': LPS,
        ']': LPE,
    }
    
    filtered = [cmd_map[c] for c in bf_code if c in cmd_map]
    
    def build_nested(lst):
        if not lst:
            return None
        return (lst[0], build_nested(lst[1:]))

    return build_nested(filtered)


INC = iota()
DEC = iota()
SHR = iota()
SHL = iota()
LPS = iota()
LPE = iota()

type Cons[T] = tuple[T, Optional[Cons[T]]]

# outputs the cells
def interpret(
    instruction_set: Cons[int],
    data_array: list[int] = [0] * 100,
    ptr = 0,
    loop_stack: list[int] = []
) -> list[int]:
    if instruction_set is None:
        return data_array

    (instr, next_instr) = instruction_set

    if instr == INC: data_array[ptr] += 1
    elif instr == DEC: data_array[ptr] -= 1
    elif instr == SHR: ptr += 1
    elif instr == SHL: ptr -= 1
    elif instr == LPS: loop_stack += [(next_instr, ptr)]
    elif instr == LPE:
        (loop_instrs, condition_ptr) = loop_stack[-1]

        if data_array[condition_ptr] == 0:
            loop_stack.pop()
            return interpret(next_instr, data_array, ptr, loop_stack)
        else:
            return interpret(loop_instrs, data_array, ptr, loop_stack)

    return interpret(next_instr, data_array, ptr, loop_stack)


instruction_set = bf("+++++[>++++<-]")
data_array = interpret(instruction_set)
print(data_array[:5])
