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


"""gdbinit script"""


import os, pathlib, sys


from elftools.elf.elffile import ELFFile
from elftools.elf.sections import SymbolTableSection


CRT0_SIZE_SYMNAME = '__metadataOff'
GOT_SIZE_SYMNAME = '__gotSize'
FLASH_RAM_SIZE_SYMNAME = '__romRamSize'


def usage():
    """Prints how to to use the script and exit"""
    print(f'usage: {sys.argv[0]} CRT0 ELF SYMBOL RELOCATION')
    sys.exit(1)


def die(message):
    """Prints error message and exit"""
    print(f'{sys.argv[0]}: {message}', file=sys.stderr)
    sys.exit(1)


def get_st_value(sh, symname):
    """Returns the st_value of the symbol"""
    symbols = sh.get_symbol_by_name(symname)
    if not symbols:
        die(f'{symname}: no symbol with this name')
    if len(symbols) > 1:
        die(f'{symname}: several symbols with this name')
    return symbols[0].entry['st_value']


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


if __name__ == '__main__':
    try:
        with open(sys.argv[1], 'rb') as f:
            sh = get_symtab_section(ELFFile(f))
            crt0_size = get_st_value(sh, CRT0_SIZE_SYMNAME)
            crt0_path = str(pathlib.Path(sys.argv[1]).resolve())
    except FileNotFoundError:
        die(f'{sys.argv[1]}: not found')
    except IndexError:
        usage()

    try:
        with open(sys.argv[2], 'rb') as f:
            sh = get_symtab_section(ELFFile(f))
            got_size = get_st_value(sh, GOT_SIZE_SYMNAME)
            flash_ram_size = get_st_value(sh, FLASH_RAM_SIZE_SYMNAME)
            elf_path = str(pathlib.Path(sys.argv[2]).resolve())
    except FileNotFoundError:
        die(f'{sys.argv[2]}: not found')
    except IndexError:
        usage()
    
    try:
        symbols_size = pathlib.Path(sys.argv[3]).stat().st_size
    except FileNotFoundError:
        die(f'{sys.argv[3]}: not found')
    except IndexError:
        usage()

    try:
        relocation_size = pathlib.Path(sys.argv[4]).stat().st_size
    except FileNotFoundError:
        die(f'{sys.argv[4]}: not found')
    except IndexError:
        usage()

    flash_off = crt0_size + symbols_size + relocation_size
    flash_ram_off = got_size
    ram_off = flash_ram_off + flash_ram_size

    crt0_addr = f'$fbase'
    flash_addr = f'$fbase+{flash_off}'
    got_addr = f'$rbase'
    flash_ram_addr = f'$rbase+{flash_ram_off}'
    ram_addr = f'$rbase+{ram_off}'

    sys.stdout.write(
        '# You need to define the two variables `$fbase\' and `$rbase\',\n'
        '# which respectively contain the address of the location where\n'
        '# the binary was loaded into Flash and the address of the\n'
        '# location where the data was relocated into RAM.\n\n'

        '# set $fbase = ...\n'
        '# set $rbase = ...\n\n'

        f'set $rom_base = {flash_addr}\n\n'

        f'add-symbol-file {crt0_path}\\\n'
        f'    -s .text {crt0_addr}\n\n'

        f'add-symbol-file {elf_path}\\\n'
        f'    -s .rom {flash_addr}\\\n'
        f'    -s .got {got_addr}\\\n'
        f'    -s .rom.ram {flash_ram_addr}\\\n'
        f'    -s .ram {ram_addr}\n'
    )

    sys.exit(0)
