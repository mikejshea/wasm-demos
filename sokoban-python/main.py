from wasmer import engine, Store, Module, Instance
# from wasmer_compiler_cranelift import Compiler
from wasmer_compiler_llvm import Compiler
import os
import argparse
import sys

path = '../sokoban-wasm/pkg/sokoban_wasm_bg.wasm'
# state = 'NNWWWWWW|WWW....W|W...B..W|W......W|W.YB...W|W......W|W..S...W|W......W|WWWWWWWW'

__dir__ = os.path.dirname(os.path.realpath(__file__))


class WasmStringInput:
    def __init__(self, input_string, instance):
        self.instance = instance
        self.input_string = bytes(input_string, 'utf-8')
        self.length_of_input = len(self.input_string) + 1
        self.input_pointer = instance.exports.allocate(self.length_of_input)
        self.memory = instance.exports.memory.uint8_view(self.input_pointer)
        self.memory[0:self.length_of_input] = self.input_string
        self.memory[self.length_of_input]

    def deallocate(self):
        self.instance.exports.deallocate(self.input_pointer, self.length_of_input)


class WasmIntResult:
    def __init__(self, output_pointer, instance):
        self.instance = instance
        self.output_pointer = output_pointer
        memory = instance.exports.memory.uint8_view(output_pointer)
        memory_length = len(memory)
        output = []
        nth = 0
        while nth < memory_length:
            byte = memory[nth]
            if byte == 0:
                break
            output.append(byte)
            nth += 1
        self.length_of_output = nth
        self.result = bytes(output).decode()

    def deallocate(self):
        self.instance.exports.deallocate(self.output_pointer, self.length_of_output)


class WasmStringResult:
    def __init__(self, output_pointer, instance):
        self.instance = instance
        self.output_pointer = output_pointer
        memory = instance.exports.memory.uint8_view(output_pointer)
        memory_length = len(memory)
        output = []
        nth = 0
        while nth < memory_length:
            byte = memory[nth]
            if byte == 0:
                break
            output.append(byte)
            nth += 1
        self.length_of_output = nth
        self.result = bytes(output).decode()

    def deallocate(self):
        self.instance.exports.deallocate(self.output_pointer, self.length_of_output)


def print_board(board):
    replacements = {
        'P': ' ☺ ',
        'W': '███',
        'S': ' ● ',
        'B': ' ⊠ ',
        '.': '   ',
        'Y': ' ☹ ',
        'Z': ' \u29C7 '
    }
    out_board = board
    for c in replacements:
        out_board = out_board.replace(c, '{new_char}'.format(new_char=replacements[c]))

    board_lines = out_board.split('|')
    print('')
    for line in board_lines:
        print(line)
    print('')


def move_player(instance, board_state, move_direction):
    board = WasmStringInput(board_state, instance)
    direction = WasmStringInput(move_direction, instance)
    wasm_result = WasmStringResult(instance.exports.c_move(board.input_pointer, direction.input_pointer), instance)
    new_state = wasm_result.result
    board.deallocate()
    direction.deallocate()
    wasm_result.deallocate()
    return new_state


def check_win(instance, board_state):
    board = WasmStringInput(board_state, instance)
    wasm_result = WasmStringResult(instance.exports.c_win_state(board.input_pointer), instance)
    result = wasm_result.result
    board.deallocate()
    wasm_result.deallocate()
    if result != "win":
        return False
    else:
        return True


def main(argv):
    valid_input = {
        'w': 'U',
        'a': 'L',
        's': 'D',
        'd': 'R'
    }
    parser = argparse.ArgumentParser()
    parser.add_argument("level", help="Select level to play, between 1 and 5", type=int, choices=range(1, 6))
    parser.add_argument("board", help="Select board to play, between 1 and 5", type=int, choices=range(1, 6))

    args = parser.parse_args()

    # Instantiates the module.
    store = Store(engine.JIT(Compiler))
    module = Module(store, open(path, 'rb').read())
    instance = Instance(module)

    wasm_result = WasmStringResult(instance.exports.c_get_level(args.level, args.board), instance)
    state = wasm_result.result
    wasm_result.deallocate()

    # state = "WWWWWWW|W.....W|WS.P..W|W.....W|WWWWWWW"

    print_board(state)
    user_input = ''
    win_state = False
    while win_state is False:
        user_input = input("Enter `w', `a`, `s`, `d` to move, `q` to exit.\n")
        if user_input == 'q':
            exit()

        if user_input in valid_input:
            print("Moving...")
            state = move_player(instance, state, valid_input[user_input])
        print_board(state)
        win_state = check_win(instance, state)
        WasmStringResult(instance.exports.c_get_level(args.level, args.board), instance)
        if win_state:
            print("Winner!")


if __name__ == '__main__':
    main(sys.argv[1:])
