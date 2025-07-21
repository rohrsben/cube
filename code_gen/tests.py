from cube import cube_str

slices = "mes"
rotations = 'xyz'
turns = 'udrlfb'

cubes = {
    'm': {
        'init': cube_str(3, {
            'top': 'd',
            'left': 'd',
            'front': 'rbg gwy orb',
            'right': 'd',
            'back': 'd',
            'bottom': 'd'
        }),
        'first': cube_str(3, {
            'top': 'bgg',
            'left': 'd',
            'front': 'ybg ywy yrb',
            'right': 'd',
            'back': 'yyo yyg yyr',
            'bottom': 'gbb'
        }),
        'last': cube_str(3, {
            'top': 'ggb',
            'left': 'd',
            'front': 'rby gwy ory',
            'right': 'd',
            'back': 'byy y gyy',
            'bottom': 'bbg'
        })
    },
    'e': {
        'init': cube_str(3, {
            'top': 'd',
            'left': 'd',
            'front': 'rbg gwy orb',
            'right': 'd',
            'back': 'd',
            'bottom': 'd'
        }),
        'first': cube_str(3, {
            'top': 'd',
            'left': 'o r r',
            'front': 'y gwy orb',
            'right': 'r o o',
            'back': 'rbg y y',
            'bottom': 'd'
        }),
        'last': cube_str(3, {
            'top': 'd',
            'left': 'r r o',
            'front': 'rbg gwy yyy',
            'right': 'o o r',
            'back': 'y y orb',
            'bottom': 'd'
        })
    },
    's': {
        'init': cube_str(3, {
            'top': 'd',
            'left': 'd',
            'right': 'rbg gwy orb',
            'front': 'd',
            'back': 'd',
            'bottom': 'd'
        }),
        'first': cube_str(3, {
            'top': 'g g b',
            'left': 'rro rrg r',
            'front': 'd',
            'right': 'rbg rwy rrb',
            'back': 'd',
            'bottom': 'g b b'
        }),
        'last': cube_str(3, {
            'top': 'b g g',
            'left': 'brr yrr grr',
            'front': 'd',
            'right': 'rbr gwr orr',
            'back': 'd',
            'bottom': 'b b g'
        })
    },

    'x': cube_str(2, {
        'top': 'b',
        'left': 'r',
        'front': 'y',
        'right': 'o',
        'bottom': 'g',
        'back': 'w'
    }),
    'y': cube_str(2, {
        'top': 'g',
        'left': 'o',
        'front': 'y',
        'right': 'r',
        'bottom': 'b',
        'back': 'w'
    }),
    'z': cube_str(2, {
        'top': 'b',
        'left': 'o',
        'front': 'w',
        'right': 'r',
        'bottom': 'g',
        'back': 'y'
    }),

    'u': cube_str(2, {
        'top': 'd',
        'left': 'o r',
        'front': 'y w',
        'right': 'r o',
        'bottom': 'd',
        'back': 'w y'
    }),
    'd': cube_str(2, {
        'top': 'd',
        'left': 'r o',
        'front': 'w y',
        'right': 'o r',
        'bottom': 'd',
        'back': 'y w'
    }),
    'r': cube_str(2, {
        'top': 'gb',
        'left': 'd',
        'front': 'wy',
        'right': 'd',
        'bottom': 'bg',
        'back': 'wy'
    }),
    'l': cube_str(2, {
        'top': 'bg',
        'left': 'd',
        'front': 'yw',
        'right': 'd',
        'bottom': 'gb',
        'back': 'yw'
    }),
    'f': cube_str(2, {
        'top': 'g b',
        'left': 'ro',
        'front': 'd',
        'right': 'ro',
        'bottom': 'g b',
        'back': 'd'
    }),
    'b': cube_str(2, {
        'top': 'b g',
        'left': 'or',
        'front': 'd',
        'right': 'or',
        'bottom': 'b g',
        'back': 'd'
    }),
}

def make_rotate(move, prime):
    s = '    '
    p_str = '_prime' if prime else ''

    lines = [
        f'{s}#[test]',
        f'{s}fn rotate_{move}{p_str}() {{',

        f'{s*2}let mut result = Cube::new(2);',
        f'{s*2}result.rotate_{move}{p_str}();',
        f'{s*2}result.rotate_{move}{p_str}();\n',

        f'{s*2}let expected = Cube {{',
        cubes[move],
        f'{s*2}}};\n',

        f'{s*2}check_eq(&result, &expected);',

        f'{s}}}'
    ]

    for line in lines:
        print(line)


def make_turn(move, prime):
    s = '    '
    p_str = '_prime' if prime else ''

    lines = [
        f'{s}#[test]',
        f'{s}fn turn_{move}{p_str}() {{',

        f'{s*2}let mut result = Cube::new(2);',
        f'{s*2}result.turn_{move}{p_str}();',
        f'{s*2}result.turn_{move}{p_str}();\n',

        f'{s*2}let expected = Cube {{',
        cubes[move],
        f'{s*2}}};\n',

        f'{s*2}check_eq(&result, &expected);',

        f'{s}}}'
    ]

    for line in lines:
        print(line)

def make_slice(move, prime, layer):
    s = '    '
    p_str = '_prime' if prime else ''
    l_str = 'first' if layer == 0 else 'last'

    lines = [
        f'{s}#[test]',
        f'{s}fn slice_{move}{p_str}_{l_str}() {{',

        f'{s*2}let mut result = Cube {{',
        cubes[move]['init'],
        f'{s*2}}};',
        f'{s*2}let expected_mod4 = result.clone();\n',

        f'{s*2}result.slice_{move}{p_str}({layer});',
        f'{s*2}result.slice_{move}{p_str}({layer});\n',

        f'{s*2}let expected = Cube {{',
        cubes[move][l_str],
        f'{s*2}}};\n',

        f'{s*2}check_eq(&result, &expected);\n',

        f'{s*2}result.slice_{move}{p_str}({layer});',
        f'{s*2}result.slice_{move}{p_str}({layer});\n',

        f'{s*2}check_eq(&result, &expected_mod4);',

        f'{s}}}'
    ]

    for line in lines:
        print(line)

def make_slice_combo(move):
    s = '    '
    lines = [
        f'{s}#[test]',
        f'{s}fn slice_{move}_and_prime() {{',

        f'{s*2}let mut result = Cube {{',
        cubes[move]['init'],
        f'{s*2}}};\n',

        f'{s*2}let expected = result.clone();\n',

        f'{s*2}result.slice_{move}(0);',
        f'{s*2}result.slice_{move}_prime(0);\n',

        f'{s*2}check_eq(&result, &expected);\n',

        f'{s*2}let mut result_prime = result.clone();\n',

        f'{s*2}result.slice_{move}(0);',
        f'{s*2}result.slice_{move}(0);\n',

        f'{s*2}result_prime.slice_{move}_prime(0);',
        f'{s*2}result_prime.slice_{move}_prime(0);\n',

        f'{s*2}check_eq(&result, &result_prime);',

        f'{s}}}'
    ]

    for line in lines:
        print(line)

def print_variations(move):
    for is_prime in [False, True]:
        make_turn(move, is_prime)
        print()

for m in turns:
    print_variations(m)
