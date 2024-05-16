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


"""relocation script"""


import sys


from elftools.elf.elffile import ELFFile
from elftools.elf.relocation import RelocationSection
from elftools.elf.enums import ENUM_RELOC_TYPE_ARM


def usage():
    """Print how to to use the script and exit"""
    print(f'usage: {sys.argv[0]} ELF')
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


def get_r_offsets(sh):
    """Returns all r_offset fields in a relocation section"""
    xs = bytearray(to_word(sh.num_relocations()))
    for i, entry in enumerate(sh.iter_relocations()):
        if get_r_type(entry['r_info']) != ENUM_RELOC_TYPE_ARM['R_ARM_ABS32']:
            die(f'{name}: entry {i}: unsupported relocation type')
        xs += to_word(entry['r_offset'])
    return xs


def get_relocation_section(elf, relname):
    """Returns a RelocationSection instance of type SHT_REL"""
    sh = elf.get_section_by_name(relname)
    if not sh:
        return None
    if not isinstance(sh, RelocationSection):
        die(f'{relname}: not a relocation section')
    if sh.is_RELA():
        die(f'{relname}: unsupported RELA section')
    return sh


if __name__ == '__main__':
    try:
        with open(sys.argv[1], 'rb') as f:
            sh = get_relocation_section(ELFFile(f), '.rel.rom.ram')
            xs = get_r_offsets(sh) if sh is not None else to_word(0)
    except FileNotFoundError:
        die(f'{sys.argv[1]}: not found')
    except IndexError:
        usage()
    sys.stdout.buffer.write(xs)
    sys.exit(0)
