/*******************************************************************************/
/*  © Université de Lille, The Pip Development Team (2015-2024)                */
/*                                                                             */
/*  This software is a computer program whose purpose is to run a minimal,     */
/*  hypervisor relying on proven properties such as memory isolation.          */
/*                                                                             */
/*  This software is governed by the CeCILL license under French law and       */
/*  abiding by the rules of distribution of free software.  You can  use,      */
/*  modify and/ or redistribute the software under the terms of the CeCILL     */
/*  license as circulated by CEA, CNRS and INRIA at the following URL          */
/*  "http://www.cecill.info".                                                  */
/*                                                                             */
/*  As a counterpart to the access to the source code and  rights to copy,     */
/*  modify and redistribute granted by the license, users are provided only    */
/*  with a limited warranty  and the software's author,  the holder of the     */
/*  economic rights,  and the successive licensors  have only  limited         */
/*  liability.                                                                 */
/*                                                                             */
/*  In this respect, the user's attention is drawn to the risks associated     */
/*  with loading,  using,  modifying and/or developing or reproducing the      */
/*  software by the user in light of its specific status of free software,     */
/*  that may mean  that it is complicated to manipulate,  and  that  also      */
/*  therefore means  that it is reserved for developers  and  experienced      */
/*  professionals having in-depth computer knowledge. Users are therefore      */
/*  encouraged to load and test the software's suitability as regards their    */
/*  requirements in conditions enabling the security of their systems and/or   */
/*  data to be ensured and,  more generally, to use and operate it in the      */
/*  same conditions as regards security.                                       */
/*                                                                             */
/*  The fact that you are presently reading this means that you have had       */
/*  knowledge of the CeCILL license and that you accept its terms.             */
/*******************************************************************************/

OUTPUT_FORMAT(
	"elf32-littlearm",
	"elf32-littlearm",
	"elf32-littlearm"
)

OUTPUT_ARCH(arm)

ENTRY(start)

SECTIONS
{
	. = 0;


        /*
	 * The '.rom' output section collects all
	 * input sections that need to remain in ROM.
	 */
	.rom :
	{
		. = ALIGN( 4 ) ;
		__romStart = . ;
		*(.text) *(.text.*)
		. = ALIGN( 4 ) ;
		__codeEnd = . ;
		*(.rodata)  *(.rodata.*) 
		. = ALIGN( 4 ) ;

		__romEnd = . ;
	}

	__romSize = __romEnd - __romStart;

	/*
	 * The section '.got' must be placed rigth
	 * after the '.rom' section in order to save
	 * ROM space.
	 */
	.got :
	{
		. = ALIGN( 4 ) ;
		__gotStart = . ;

		*(.got) *(.got.*)
		. = ALIGN( 4 ) ;

		__gotEnd = . ;
	}

	__gotSize = __gotEnd - __gotStart;

	/*
	 * The '.rom.ram' output section collects all
	 * input sections that need to be copied from
	 * ROM to RAM.
	 */
	.rom.ram :
	{
		. = ALIGN( 4 ) ;
		__romRamStart = . ;

		*(.data) *(.data.*)
		. = ALIGN( 4 ) ;
		__romRamEnd = . ;
	}


	__romRamSize = __romRamEnd - __romRamStart;

	/*
	 * The '.ram' output section collects all
	 * input sections that need to be set to zero
	 * in RAM.
	 */
	.ram (NOLOAD) :
	{
		. = ALIGN( 4 ) ;
		__ramStart = . ;

		*(.bss .bss.* COMMON)
		. = ALIGN( 4 ) ;

		__ramEnd = . ;
	}
	.ARM.exidx :
	{
		. = ALIGN( 4 );
		*(.ARM.exidx .ARM.exidx.*);
	}
	__ramSize = __ramEnd - __ramStart;
}
