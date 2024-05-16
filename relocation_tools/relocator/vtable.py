#!/usr/bin/env python3
###############################################################################
#  © Université de Lille, The Pip Development Team (2015-2024)                #
#                                                                             #
#  This software is a computer program whose purpose is to run a minimal,     #
#  hypervisor relying on proven properties such as memory isolation.          #
#                                                                             #
#  This software is governed by the CeCILL license under French law and       #
#  abiding by the rules of distribution of free software.  You can  use,      #
#  modify and/ or redistribute the software under the terms of the CeCILL     #
#  license as circulated by CEA, CNRS and INRIA at the following URL          #
#  "http://www.cecill.info".                                                  #
#                                                                             #
#  As a counterpart to the access to the source code and  rights to copy,     #
#  modify and redistribute granted by the license, users are provided only    #
#  with a limited warranty  and the software's author,  the holder of the     #
#  economic rights,  and the successive licensors  have only  limited         #
#  liability.                                                                 #
#                                                                             #
#  In this respect, the user's attention is drawn to the risks associated     #
#  with loading,  using,  modifying and/or developing or reproducing the      #
#  software by the user in light of its specific status of free software,     #
#  that may mean  that it is complicated to manipulate,  and  that  also      #
#  therefore means  that it is reserved for developers  and  experienced      #
#  professionals having in-depth computer knowledge. Users are therefore      #
#  encouraged to load and test the software's suitability as regards their    #
#  requirements in conditions enabling the security of their systems and/or   #
#  data to be ensured and,  more generally, to use and operate it in the      #
#  same conditions as regards security.                                       #
#                                                                             #
#  The fact that you are presently reading this means that you have had       #
#  knowledge of the CeCILL license and that you accept its terms.             #
###############################################################################


"""vtable script"""


import pathlib
import sys


from elftools.elf.elffile import ELFFile
from elftools.elf.relocation import RelocationSection
from elftools.elf.sections import Section
from elftools.elf.sections import SymbolTableSection
from elftools.elf.enums import ENUM_RELOC_TYPE_ARM


def usage():
    """Print how to to use the script and exit"""
    print(f'usage: {sys.argv[0]} ELF OFFSET CRT0 SYMBOLS RELOCATION OUTPUT')
    sys.exit(1)


def die(message):
    """Print error message and exit"""
    print(f'{sys.argv[0]}: {message}', file=sys.stderr)
    sys.exit(1)


def to_word(x):
    """Convert a python integer to a LE 4-bytes bytearray"""
    return x.to_bytes(4, byteorder='little')


def get_r_type(r_info):
    """Returns the relocation type from r_info"""
    return r_info & 0xff


def get_st_value(sh, symname):
    """Returns the st_value of the symbol"""
    symbols = sh.get_symbol_by_name(symname)
    if not symbols:
        die(f'{symname}: no symbol with this name')
    if len(symbols) > 1:
        die(f'{symname}: several symbols with this name')
    return to_word(symbols[0].entry['st_value'])


def get_symtab_section(elf):
    """Returns a SymbolTableSection instance of type SHT_SYMTAB"""
    sh = elf.get_section_by_name('.symtab')
    if not sh:
        die(f'.symtab: no section with this name')
    if not isinstance(sh, SymbolTableSection):
        die(f'.symtab: not a symbol table section')
    if sh['sh_type'] != 'SHT_SYMTAB':
        die(f'.symtab: not a SHT_SYMTAB section')
    return sh


def patch_vtable(text, rel, sym, offset):
    """Patch vtables in a relocation section"""
    data = bytearray(text.data())
    rom_start = int.from_bytes(get_st_value(sym, '__romStart'), byteorder='little')
    code_end = int.from_bytes(get_st_value(sym, '__codeEnd'), byteorder='little')
    for i, entry in enumerate(rel.iter_relocations()):
        if get_r_type(entry['r_info']) == ENUM_RELOC_TYPE_ARM['R_ARM_ABS32']:
            symbol_value = sym.get_symbol(((entry['r_info'] >> 8) & 0xffffff)).entry['st_value']
            r_offset = entry['r_offset']
            if (rom_start <= symbol_value <= code_end) and not(rom_start <= r_offset <= code_end):
                name = sym.get_symbol(entry['r_info_sym']).name
                if name:
                    new_val = offset + symbol_value
                    val = to_word(new_val)
                    data[r_offset] = val[0]
                    data[r_offset+1] = val[1]
                    data[r_offset+2] = val[2]
                    data[r_offset+3] = val[3]
                    print(name)
                    print(f'{symbol_value:x} {new_val:x} {r_offset:x}')
    return data



def get_text_section(elf, name):
    """"""
    sh = elf.get_section_by_name(name)
    if not sh:
        return None
    if not isinstance(sh, Section):
        die(f'{name}: not a section')
    return sh


def get_relocation_section(elf, name):
    """Returns a RelocationSection instance of type SHT_REL"""
    sh = elf.get_section_by_name(name)
    if not sh:
        return None
    if not isinstance(sh, RelocationSection):
        die(f'{name}: not a relocation section')
    if sh.is_RELA():
        die(f'{name}: unsupported RELA section')
    return sh


if __name__ == '__main__':
    try:
        crt0_size = pathlib.Path(sys.argv[3]).stat().st_size
        symbols_size = pathlib.Path(sys.argv[4]).stat().st_size
        relocation_size = pathlib.Path(sys.argv[5]).stat().st_size
        offset = int(sys.argv[2], 16) + crt0_size + symbols_size + relocation_size
        with open(sys.argv[1], 'rb') as f:
            elf = ELFFile(f)
            sym = get_symtab_section(elf)
            text = get_text_section(elf, '.rom')
            rel = get_relocation_section(elf, '.rel.rom')
            data = patch_vtable(text, rel, sym, offset)
            data += elf.get_section_by_name('.rom.ram').data()
            data += elf.get_section_by_name('.ARM.exidx').data()
            with open(sys.argv[6], 'wb') as f:
                f.write(data)
    except FileNotFoundError as e:
        die(e)
    except IndexError:
        usage()
    sys.exit(0)
