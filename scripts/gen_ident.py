import itertools, string

t_invalid = 0
t_ident = 1

def table_group_assign(table, group, value):
    for x in group:
        table[ord(x)] = value

def render_table(name, indent, t):
    header = "const %s: [u8; 256] = [" % (name.upper())
    footer = "];"
    result = [header]
    current_line = indent

    for i in range(0, 256):
        s = "%d, " % (t[i])
        current_line += s

        if len(current_line) >= 100:
            result.append(current_line)
            current_line = indent

    result.append(footer)

    for line in result:
        print(line.rstrip(" "))

table = [x for x in itertools.repeat(t_invalid, 256)]

table_group_assign(table, string.ascii_lowercase, t_ident)
table_group_assign(table, string.ascii_uppercase, t_ident)
table_group_assign(table, "_", t_ident)
render_table("is_ident_table", "    ", table)
