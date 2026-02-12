#!/usr/bin/env python3
"""
header2yaml - Convert SDK C header files to chiptool yaml transform

Usage:
    # Single peripheral
    ./header2yaml.py -i bt_rfc.h -n BT_RFC -a 0x40082800 -o bt_rfc.yaml

    # Auto-detect peripheral name (inferred from typedef)
    ./header2yaml.py -i bt_rfc.h -a 0x40082800 -o bt_rfc.yaml

    # Batch processing (using config file)
    ./header2yaml.py --config peripherals.json -o output/

Config file format (JSON):
    [
        {"header": "bt_rfc.h", "name": "BT_RFC", "base": "0x40082800"},
        {"header": "bt_phy.h", "name": "BT_PHY", "base": "0x40084000"}
    ]
"""

import argparse
import json
import re
import sys
from collections import defaultdict
from pathlib import Path


def parse_header(filename, peripheral_name=None):
    """
    Parse a header file, return (peripheral_name, register_list, fields_dict)

    Supported formats:
    - typedef struct { ... } XXX_TypeDef;
    - __IO uint32_t REG_NAME;
    - __IO uint32_t REG_NAME[N];
    - #define XXX_REG_FIELD_Pos (N)
    - #define XXX_REG_FIELD_Msk (0xNUL << XXX_REG_FIELD_Pos)
    """
    with open(filename, 'r', encoding='utf-8', errors='ignore') as f:
        content = f.read()

    # Auto-detect peripheral name (inferred from typedef)
    if peripheral_name is None:
        typedef_match = re.search(r'typedef struct\s*\{[^}]+\}\s*(\w+)_TypeDef', content)
        if typedef_match:
            peripheral_name = typedef_match.group(1)
        else:
            # Try to infer from filename
            peripheral_name = Path(filename).stem.upper()

    # Find struct definition
    typedef_name = f'{peripheral_name}_TypeDef'
    struct_pattern = rf'typedef struct\s*\{{([^}}]+)\}}\s*{typedef_name}'
    struct_match = re.search(struct_pattern, content)

    if not struct_match:
        # Try a more relaxed match
        struct_pattern = rf'typedef struct\s*\{{([^}}]+)\}}\s*\w*{peripheral_name}\w*'
        struct_match = re.search(struct_pattern, content, re.IGNORECASE)

    if not struct_match:
        print(f"Warning: cannot find {typedef_name} struct", file=sys.stderr)
        return peripheral_name, [], {}

    struct_body = struct_match.group(1)
    registers = []
    offset = 0

    # Parse register definitions
    for line in struct_body.split('\n'):
        line = line.strip()

        # Single register: __IO uint32_t REG_NAME;
        single_match = re.match(r'__IO\s+uint32_t\s+(\w+)\s*;', line)
        if single_match:
            reg_name = single_match.group(1)
            if not reg_name.upper().startswith('RSVD') and not reg_name.upper().startswith('RESERVED'):
                registers.append({'name': reg_name, 'offset': offset, 'count': 1})
            offset += 4
            continue

        # Array register: __IO uint32_t REG_NAME[N];
        array_match = re.match(r'__IO\s+uint32_t\s+(\w+)\[(\d+)\]\s*;', line)
        if array_match:
            reg_name = array_match.group(1)
            count = int(array_match.group(2))
            if not reg_name.upper().startswith('RSVD') and not reg_name.upper().startswith('RESERVED'):
                registers.append({'name': reg_name, 'offset': offset, 'count': count})
            offset += 4 * count
            continue

        # 16-bit register: __IO uint16_t REG_NAME;
        single16_match = re.match(r'__IO\s+uint16_t\s+(\w+)\s*;', line)
        if single16_match:
            reg_name = single16_match.group(1)
            if not reg_name.upper().startswith('RSVD') and not reg_name.upper().startswith('RESERVED'):
                registers.append({'name': reg_name, 'offset': offset, 'count': 1, 'size': 16})
            offset += 2
            continue

        # 8-bit register: __IO uint8_t REG_NAME;
        single8_match = re.match(r'__IO\s+uint8_t\s+(\w+)\s*;', line)
        if single8_match:
            reg_name = single8_match.group(1)
            if not reg_name.upper().startswith('RSVD') and not reg_name.upper().startswith('RESERVED'):
                registers.append({'name': reg_name, 'offset': offset, 'count': 1, 'size': 8})
            offset += 1
            continue

    # Build register name set (for matching bitfields)
    reg_set = set(r['name'].upper() for r in registers)
    fields_by_reg = defaultdict(list)

    # Parse bitfield definitions
    prefix = peripheral_name.upper()

    # Match _Pos definitions
    pos_pattern = rf'#define\s+{prefix}_(\S+)_Pos\s+\(?(\d+)U?\)?'
    pos_lines = re.findall(pos_pattern, content)

    # Match _Msk definitions
    msk_pattern = rf'#define\s+{prefix}_(\S+)_Msk\s+\(?(0x[0-9A-Fa-f]+)U?L?'
    msk_lines = re.findall(msk_pattern, content)
    msk_dict = {name: int(msk, 16) for name, msk in msk_lines}

    # Match bitfields to registers
    for full_name, pos_str in pos_lines:
        pos = int(pos_str)
        found_reg = None
        found_field = None

        # Try from longest match first
        parts = full_name.split('_')
        for i in range(len(parts), 0, -1):
            candidate_reg = '_'.join(parts[:i])
            if candidate_reg in reg_set:
                found_reg = candidate_reg
                found_field = '_'.join(parts[i:])
                break

        if found_reg and found_field:
            # Calculate bit width from Msk
            msk = msk_dict.get(full_name, 0x1)
            # Compute actual mask width after removing shift
            actual_msk = msk >> pos if pos > 0 else msk
            width = actual_msk.bit_length() if actual_msk > 0 else 1

            fields_by_reg[found_reg].append({
                'name': found_field.lower(),
                'bit_offset': pos,
                'bit_size': width,
            })

    # Sort by bit offset
    for reg in fields_by_reg:
        fields_by_reg[reg].sort(key=lambda x: x['bit_offset'])

    return peripheral_name, registers, dict(fields_by_reg)


def to_pascal_case(name):
    """VCO_REG1 -> VcoReg1"""
    parts = name.lower().split('_')
    return ''.join(part.capitalize() for part in parts)


def to_snake_case(name):
    """VCO_REG1 -> vco_reg1"""
    return name.lower()


def generate_yaml(peripheral_name, base_addr, registers, fields_by_reg, module_name=None):
    """Generate chiptool yaml transform"""
    if module_name is None:
        module_name = peripheral_name.lower()

    lines = []
    lines.append(f"# {peripheral_name} Peripheral Definition")
    lines.append(f"# Auto-generated from SDK header file")
    lines.append(f"# {len(registers)} registers, {sum(len(f) for f in fields_by_reg.values())} fields")
    lines.append("")
    lines.append("transforms:")
    lines.append("")
    lines.append(f"  - !AddPeripherals")
    lines.append("      devices: .*")
    lines.append("      peripherals:")
    lines.append(f"        - name: {peripheral_name}")
    lines.append(f"          base_address: {hex(base_addr)}")
    lines.append(f"          block: {module_name}::{to_pascal_case(peripheral_name)}")
    lines.append("")
    lines.append("  - !Add")
    lines.append("      ir:")
    lines.append(f"        block/{module_name}::{to_pascal_case(peripheral_name)}:")
    lines.append("          items:")

    for reg in registers:
        reg_name = reg['name']
        reg_offset = reg['offset']
        count = reg['count']

        if count == 1:
            lines.append(f"            - name: {to_snake_case(reg_name)}")
            lines.append(f"              byte_offset: {hex(reg_offset)}")
            lines.append(f"              fieldset: {module_name}::regs::{to_pascal_case(reg_name)}")
        else:
            lines.append(f"            - name: {to_snake_case(reg_name)}")
            lines.append(f"              byte_offset: {hex(reg_offset)}")
            lines.append(f"              fieldset: {module_name}::regs::{to_pascal_case(reg_name)}")
            lines.append(f"              array:")
            lines.append(f"                len: {count}")
            lines.append(f"                stride: 4")

    lines.append("")

    for reg in registers:
        reg_name = reg['name']
        reg_upper = reg_name.upper()
        fieldset_name = to_pascal_case(reg_name)
        fields = fields_by_reg.get(reg_upper, [])

        lines.append(f"        fieldset/{module_name}::regs::{fieldset_name}:")
        lines.append(f"          description: \"{reg_name}\"")
        lines.append("          fields:")

        if fields:
            for field in fields:
                lines.append(f"            - name: {field['name']}")
                lines.append(f"              bit_offset: {field['bit_offset']}")
                lines.append(f"              bit_size: {field['bit_size']}")
        else:
            lines.append("            - name: value")
            lines.append("              bit_offset: 0")
            lines.append("              bit_size: 32")

        lines.append("")

    return '\n'.join(lines)


def process_single(args):
    """Process a single header file"""
    peripheral_name, registers, fields = parse_header(args.input, args.name)

    if not registers:
        print(f"Error: cannot parse registers from {args.input}", file=sys.stderr)
        return False

    yaml_content = generate_yaml(
        peripheral_name,
        int(args.address, 16) if args.address.startswith('0x') else int(args.address),
        registers,
        fields,
        args.module
    )

    if args.output:
        output_path = Path(args.output)
        output_path.parent.mkdir(parents=True, exist_ok=True)
        with open(output_path, 'w') as f:
            f.write(yaml_content)
        print(f"{peripheral_name}: {len(registers)} regs, {sum(len(f) for f in fields.values())} fields -> {output_path}")
    else:
        print(yaml_content)

    return True


def process_config(args):
    """Batch process using config file"""
    with open(args.config, 'r') as f:
        config = json.load(f)

    output_dir = Path(args.output) if args.output else Path('.')
    output_dir.mkdir(parents=True, exist_ok=True)

    for item in config:
        header = item['header']
        name = item.get('name')
        base = item['base']
        module = item.get('module', name.lower() if name else None)

        peripheral_name, registers, fields = parse_header(header, name)

        if not registers:
            print(f"Warning: skipping {header}, cannot parse registers", file=sys.stderr)
            continue

        base_addr = int(base, 16) if isinstance(base, str) and base.startswith('0x') else int(base)
        yaml_content = generate_yaml(peripheral_name, base_addr, registers, fields, module)

        output_file = output_dir / f"{module or peripheral_name.lower()}.yaml"
        with open(output_file, 'w') as f:
            f.write(yaml_content)

        num_fields = sum(len(f) for f in fields.values())
        print(f"{peripheral_name}: {len(registers)} regs, {num_fields} fields -> {output_file.name}")


def main():
    parser = argparse.ArgumentParser(
        description='Convert SDK C header files to chiptool yaml transform',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  %(prog)s -i bt_rfc.h -n BT_RFC -a 0x40082800 -o bt_rfc.yaml
  %(prog)s -i usart.h -a 0x50084000    # auto-detect peripheral name
  %(prog)s --config peripherals.json -o output/
        """
    )

    parser.add_argument('-i', '--input', help='Input header file path')
    parser.add_argument('-n', '--name', help='Peripheral name (optional, auto-detected)')
    parser.add_argument('-a', '--address', help='Base address (e.g. 0x40082800)')
    parser.add_argument('-m', '--module', help='Module name (default: peripheral name in lowercase)')
    parser.add_argument('-o', '--output', help='Output file/directory path')
    parser.add_argument('-c', '--config', help='Batch processing config file (JSON)')

    args = parser.parse_args()

    if args.config:
        process_config(args)
    elif args.input and args.address:
        process_single(args)
    else:
        parser.print_help()
        sys.exit(1)


if __name__ == '__main__':
    main()
