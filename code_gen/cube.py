def default(face):
    if face == 'front':
        return 'White'
    elif face == 'top':
        return 'Green'
    elif face == 'right':
        return 'Orange'
    elif face == 'left':
        return 'Red'
    elif face == 'bottom':
        return 'Blue'
    elif face == 'back':
        return 'Yellow'

def to_color(face, input):
    if input == 'w':
        return 'White'
    elif input == 'g':
        return 'Green'
    elif input == 'o':
        return 'Orange'
    elif input == 'r':
        return 'Red'
    elif input == 'b':
        return 'Blue'
    elif input == 'y':
        return 'Yellow'
    elif input == 'd':
        return default(face)
    else:
        return 'fucked'

def parse_row(size, face, input):
    result = []

    if len(input) == 1:
        contents = f'{to_color(face, input)}; {size}'
        result.append(contents)
    else:
        for tile in input:
            tile_str = to_color(face, tile)
            result.append(tile_str)

    return f'vec![{', '.join(result)}]'

def parse_face(size, face, input, spacing):
    spacer = '    '
    as_list = input.split()

    if len(as_list) == 1:
        parsed_row = parse_row(size, face, as_list[0])
        return f'{spacer * spacing}{face}: vec![{parsed_row}; {size}],'

    parsed_rows = [parse_row(size, face, row) for row in as_list]
    return f'{spacer * spacing}{face}: vec![\n{''.join([f'{spacer * (spacing+1)}{row},\n' for row in parsed_rows])}{spacer * spacing}],'
    
# expects a dict
def parse_cube(size, input, spacing):
    return {
        'size': f'{'    '*spacing}size: {size},',
        'front': parse_face(size, 'front', input['front'], spacing),
        'top': parse_face(size, 'top', input['top'], spacing),
        'left': parse_face(size, 'left', input['left'], spacing),
        'right': parse_face(size, 'right', input['right'], spacing),
        'back': parse_face(size, 'back', input['back'], spacing),
        'bottom': parse_face(size, 'bottom', input['bottom'], spacing)
    }

def cube_str(size, input, spacing=3):
    c = parse_cube(size, input, spacing)

    s = c['size']
    f = c['front']
    t = c['top']
    l = c['left']
    r = c['right']
    ba = c['back']
    bo = c['bottom']

    return f'{s}\n{t}\n{l}\n{f}\n{r}\n{bo}\n{ba}'
