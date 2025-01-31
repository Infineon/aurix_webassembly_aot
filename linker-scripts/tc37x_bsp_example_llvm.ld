/**************************************************************************************************
 * FILE: tc37x_bsp_example.ld
 *
 * UC ARCHITECTURE:
 *
 *     TC3xx
 *
 * UC FAMILY:
 *
 *     TC37x
 *
 * DESCRIPTION:
 *
 *     Application linker file describing only elements used in BSP example 
 *     to keep the file as simple as possible.
 *
 *     It locates object sections to memory regions defined in <uC>_memory
 *     linker file via predefined ALIASES.
 *
 * FILE HISTORY:
 *
 *     V1    05.2020    RO
 *     - Base example reference
 *
 **************************************************************************************************
 * Copyright (C) 2015-2020 HighTec EDV-Systeme GmbH.  All rights reserved.
 * This is proprietary software. Any use of the software requires a written
 * license agreement with HighTec EDV-Systeme GmbH. Please contact for
 * detailed license information: info@hightec-rt.com.
 *************************************************************************************************/
 
 
/* ================================================================================================
 * PREDEFINED OUTPUT SECTIONS TO MEMORY REGION ALIASES
 * Aliases allow a quick change in memory placement
 * ==============================================================================================*/

/* BMHD sections */ 
REGION_ALIAS("BMHD_ORIG", ucb_bmhd_orig)
REGION_ALIAS("BMHD_COPY", ucb_bmhd_copy)

/* CRT0 Boot Code Start */ 
REGION_ALIAS("BOOT", int_flash0)

/* BSP Interrupt handler table in RAM */
REGION_ALIAS("BSP_ISR_HANDLERS_CPU0_", pspr_cpu0) 
REGION_ALIAS("BSP_ISR_HANDLERS_CPU1_", pspr_cpu1) 
REGION_ALIAS("BSP_ISR_HANDLERS_CPU2_", pspr_cpu2) 

/* Core0: Output sections to memory region mapping */
REGION_ALIAS("CODE_CPU0_",      int_flash0)
REGION_ALIAS("RODATA_CPU0_",    int_flash0)
REGION_ALIAS("RAMCODE_CPU0_",   pspr_cpu0)
REGION_ALIAS("DATA_DSPR_CPU0_", dspr_cpu0)
REGION_ALIAS("DATA_DLMU_CPU0_", dlmu_cpu0)

/* Core1: Output sections to memory region mapping */
REGION_ALIAS("CODE_CPU1_",      int_flash1)
REGION_ALIAS("RODATA_CPU1_",    int_flash1)
REGION_ALIAS("RAMCODE_CPU1_",   pspr_cpu1)
REGION_ALIAS("DATA_DSPR_CPU1_", dspr_cpu1)
REGION_ALIAS("DATA_DLMU_CPU1_", dlmu_cpu1)

/* Core2: Output sections to memory region mapping */
REGION_ALIAS("CODE_CPU2_",      int_flash1)
REGION_ALIAS("RODATA_CPU2_",    int_flash1)
REGION_ALIAS("RAMCODE_CPU2_",   pspr_cpu2)
REGION_ALIAS("DATA_DSPR_CPU2_", dspr_cpu2)
REGION_ALIAS("DATA_DLMU_CPU2_", dlmu_cpu2)

/* Common code and data */
REGION_ALIAS("CODE",     int_flash0)
REGION_ALIAS("RODATA",   int_flash0)
REGION_ALIAS("DATA", 	 dlmu_cpu0)
REGION_ALIAS("RAM_CODE", dlmu_cpu0)
 

/* ================================================================================================
 * PREDEFINED CONSTANTS
 * ==============================================================================================*/

/* Shared Global Stack as defined by AURIX architecture (PSW.IS = 1) 
 * the User stack is left on eventual User or RTOS control (PSW.IS = 0) 
 */
SHARED_STACK_SIZE = 32K;     

/* CSA dedicated memory region size defined as 
 * (Number of CSA regions x CSA entry size) where 256 entries is Maximum 
 */
CSA_REGIONS = 256;
CSA_SIZE    = CSA_REGIONS * 64;  

/* General HEAP is case one uses certain stdlib functions */   
__HEAP_SIZE = DEFINED (__HEAP_SIZE) ? __HEAP_SIZE : 4K;  


/* ================================================================================================
 * SECTIONS: BMHD headers
 * ==============================================================================================*/
 
SECTIONS
{
	/* Boot Mode Header - original values */
	.bmhd_0_orig 0xaf400000 : { KEEP (*(.bmhd_0_orig)) } > BMHD_ORIG
	.bmhd_1_orig 0xaf400200 : { KEEP (*(.bmhd_1_orig)) } > BMHD_ORIG
	.bmhd_2_orig 0xaf400400 : { KEEP (*(.bmhd_2_orig)) } > BMHD_ORIG
	.bmhd_3_orig 0xaf400600 : { KEEP (*(.bmhd_3_orig)) } > BMHD_ORIG

	/* Boot Mode Header - copy values */
	.bmhd_0_copy 0xaf401000 : { KEEP (*(.bmhd_0_copy)) } > BMHD_COPY
	.bmhd_1_copy 0xaf401200 : { KEEP (*(.bmhd_1_copy)) } > BMHD_COPY
	.bmhd_2_copy 0xaf401400 : { KEEP (*(.bmhd_2_copy)) } > BMHD_COPY
	.bmhd_3_copy 0xaf401600 : { KEEP (*(.bmhd_3_copy)) } > BMHD_COPY
}


/* ================================================================================================
 * SECTIONS: Application BOOT code
 * Address pointed by BMHD start address value
 * ==============================================================================================*/

SECTIONS
{
	.crt0_boot : { KEEP (*(.crt0_boot.code)) } > BOOT   
}


/* ================================================================================================
 * SECTIONS: Reset Default TRAP handlers provided by BSP 
 * the TRAP table is shared across cores
 * BTV register value after reset is 0xA0000100 (0x80000100)
 * ==============================================================================================*/

SECTIONS
{
	.bsp_trap_reset 0x80000100 :
	{
		KEEP (*(.bsp_trap_vector_table))
		KEEP (*(.bsp_trap_handlers))
	} > CODE_CPU0_
}
	
	
/* ================================================================================================
 * SECTIONS: BSP Interrupt tables
 * BSP provided Interrupt vector table and ISR Handler tables for each core
 * BSP code set BIV register to core corresponding Base address 
 * ==============================================================================================*/

SECTIONS
{
	.CPU0.bsp_isr_vector_table : { KEEP (*(.bsp_isr_vector_table_cpu0)) } > CODE_CPU0_
	.CPU1.bsp_isr_vector_table : { KEEP (*(.bsp_isr_vector_table_cpu1)) } > CODE_CPU1_
	.CPU2.bsp_isr_vector_table : { KEEP (*(.bsp_isr_vector_table_cpu2)) } > CODE_CPU2_
}

SECTIONS
{
    .CPU0.bsp_isr_ram_table (NOLOAD) : { *(.BspIsrRamTable_Cpu0) } > BSP_ISR_HANDLERS_CPU0_ 
    .CPU1.bsp_isr_ram_table (NOLOAD) : { *(.BspIsrRamTable_Cpu1) } > BSP_ISR_HANDLERS_CPU1_
    .CPU2.bsp_isr_ram_table (NOLOAD) : { *(.BspIsrRamTable_Cpu2) } > BSP_ISR_HANDLERS_CPU2_
}


/* ================================================================================================
 * SECTIONS: Cores' CSA regions 
 * Each core has its own CSA region list
 * ==============================================================================================*/

SECTIONS
{
	.CPU0.csa (NOLOAD) : ALIGN(64) { . = ALIGN(64); __CSA_BASE_CPU0_ = .; . +=  CSA_SIZE; } > DATA_DSPR_CPU0_
	.CPU1.csa (NOLOAD) : ALIGN(64) { . = ALIGN(64); __CSA_BASE_CPU1_ = .; . +=  CSA_SIZE; } > DATA_DSPR_CPU1_
    .CPU2.csa (NOLOAD) : ALIGN(64) { . = ALIGN(64); __CSA_BASE_CPU2_ = .; . +=  CSA_SIZE; } > DATA_DSPR_CPU2_
}


/* ================================================================================================
 * SECTIONS: Cores' Shared Stacks 
 * Each core has its own Shared stack area (PSW.IS = 1)
 * ==============================================================================================*/

SECTIONS
{
	.CPU0.stack (NOLOAD) : { . = ALIGN(8); __STACK_BASE_CPU0_ = .; . += SHARED_STACK_SIZE; } > DATA_DSPR_CPU0_ 
	.CPU1.stack (NOLOAD) : { . = ALIGN(8); __STACK_BASE_CPU1_ = .; . += SHARED_STACK_SIZE; } > DATA_DSPR_CPU1_ 
	.CPU2.stack (NOLOAD) : { . = ALIGN(8); __STACK_BASE_CPU2_ = .; . += SHARED_STACK_SIZE; } > DATA_DSPR_CPU2_ 
}


/* ================================================================================================
 * SECTIONS: CORE 0 dedicated sections
 * In the example they are Empty, but user might used them to play
 * with core specific placement
 * ==============================================================================================*/

SECTIONS
{
	.CPU0.code     : ALIGN(4) {  } > CODE_CPU0_
	.CPU0.ramcode  : ALIGN(4) {  } > RAMCODE_CPU0_ AT > RODATA_CPU0_
	.CPU0.rodata   : ALIGN(4) {  } > RODATA_CPU0_
	.CPU0.data     : ALIGN(4) {  } > DATA_DSPR_CPU0_ AT > RODATA_CPU0_
	.CPU0.bss      (NOLOAD) : ALIGN(4) {  } > DATA_DSPR_CPU0_
}


/* ================================================================================================
 * SECTIONS: CORE 1 dedicated sections
 * In the example they are Empty, but user might used them to play
 * with core specific placement
 * ==============================================================================================*/

SECTIONS
{
	.CPU1.code     : {  } > CODE_CPU1_
	.CPU1.ramcode  : {  } > RAMCODE_CPU1_ AT > RODATA_CPU1_
	.CPU1.rodata   : {  } > RODATA_CPU1_
	.CPU1.data     : {  } > DATA_DSPR_CPU1_ AT > RODATA_CPU1_
	.CPU1.bss      (NOLOAD) : {  } > DATA_DSPR_CPU1_
}


/* ================================================================================================
 * SECTIONS: CORE 2 dedicated sections
 * In the example they are Empty, but user might used them to play
 * with core specific placement
 * ==============================================================================================*/

SECTIONS
{
	.CPU2.code     : {  } > CODE_CPU2_
	.CPU2.ramcode  : {  } > RAMCODE_CPU2_ AT > RODATA_CPU2_
	.CPU2.rodata   : {  } > RODATA_CPU2_
	.CPU2.data     : {  } > DATA_DSPR_CPU2_ AT > RODATA_CPU2_
	.CPU2.bss      (NOLOAD) : {  } > DATA_DSPR_CPU2_
}


/* ================================================================================================
 * SECTIONS: COMMON (shared) sections
 * by default, all example sections are located here
 * ==============================================================================================*/

SECTIONS
{
	/* Code sections */
	.code :	
	{ 
	    *(.text*) 
	} > CODE
	
	/* code to execute from RAM */
	.ramcode :
	{
	    *(.ramcode*)
	} > RAM_CODE AT > RODATA
	
	/* read-only data, constants */
	.rodata : 
	{
	    *(.rodata*)
	} > RODATA

	/* Unwinding support. alloc seems to use this section */
	. = ALIGN(8);
    PROVIDE(__eh_frame = .);
	.eh_frame : { 
		KEEP (*(.eh_frame)) 
		*(.eh_frame.*) 
	} > RODATA

	/* C++ constructors */
	.ctors : ALIGN(4)
	{
		__CTOR_LIST__ = .;
		LONG((__CTOR_END__ - __CTOR_LIST__) / 4 - 2)
		KEEP (*(.ctors));
		LONG(0);
		__CTOR_END__ = .;	
	} > RODATA
	
	/* Short-addressable read-only data */
	.sdata2 : ALIGN(4)
	{
	    *(.sdata2*)
	} > RODATA
	
	/* Short-addresable read-write data - initialized */
	.sdata : ALIGN(4)
	{
	    *(.sdata*)
	} > DATA AT > RODATA
	
	/* Short-addresable read-write data - non-initialized (zeroed) */
	.sbss  (NOLOAD): ALIGN(4)
	{
	    *(.sbss*)
	} > DATA
	
	/* read-write data - initialized */
	.data : ALIGN(4)
	{
		KEEP (*(.keep_ram*));
	    *(.data*)
	} > DATA AT > RODATA

	/* read-write data - non-initialized */
	.bss (NOLOAD) : ALIGN(4)
	{
		*(.uninit*);
	     *(.bss*); 
	     *(COMMON);
	} > DATA
	
	/* HEAP area for stdlib functions */
	.heap (NOLOAD) : ALIGN(8) 
	{ 
	    . += __HEAP_SIZE;
	} > DATA
	
}


/* ================================================================================================
 * SECTIONS: CLEAR & COPY TABLES with END delimiter to support crt0 init
 * clear_table: 
 *    data memory ranges to clear to zero
 * copy_table:  
 *    data memory ranges that needs to be value initialized
 *    (init values are stored in FLASH and copied to RAM)
 * Each core has its own table to process during its init to allow multicore execution. 
 * Shared resources are inserted to Core[0] tables (the RESET core)
 * ==============================================================================================*/
SECTIONS
{
    /*-------------------------------------------------------------------------------------------*/

	.CPU0.clear_sec : ALIGN(4)
	{
		LONG(ADDR(.CPU0.bss)); LONG(SIZEOF(.CPU0.bss));
		LONG(ADDR(.sbss)); LONG(SIZEOF(.sbss)); 
		LONG(ADDR(.bss)); LONG(SIZEOF(.bss));
		LONG(ADDR(.heap)); LONG(SIZEOF(.heap));
		LONG(-1); LONG(-1);
	} > RODATA_CPU0_

	.CPU0.copy_sec : ALIGN(4)
	{
		LONG(LOADADDR(.CPU0.ramcode)); LONG(ADDR(.CPU0.ramcode)); LONG(SIZEOF(.CPU0.ramcode));
		LONG(LOADADDR(.CPU0.data)); LONG(ADDR(.CPU0.data)); LONG(SIZEOF(.CPU0.data));
		LONG(LOADADDR(.ramcode)); LONG(ADDR(.ramcode)); LONG(SIZEOF(.ramcode));
		LONG(LOADADDR(.sdata)); LONG(ADDR(.sdata)); LONG(SIZEOF(.sdata));
		LONG(LOADADDR(.data)); LONG(ADDR(.data)); LONG(SIZEOF(.data));
		LONG(-1); LONG(-1); LONG(-1);
	} > RODATA_CPU0_

	/*-------------------------------------------------------------------------------------------*/

	.CPU1.clear_sec : ALIGN(4)
	{
		LONG(ADDR(.CPU1.bss)); LONG(SIZEOF(.CPU1.bss));
		LONG(-1); LONG(-1);
	} > RODATA_CPU1_
	
	.CPU1.copy_sec : ALIGN(4)
	{
		LONG(LOADADDR(.CPU1.ramcode)); LONG(0 + ADDR(.CPU1.ramcode)); LONG(SIZEOF(.CPU1.ramcode));
		LONG(LOADADDR(.CPU1.data)); LONG(ADDR(.CPU1.data)); LONG(SIZEOF(.CPU1.data));
		LONG(-1); LONG(-1); LONG(-1);
	} > RODATA_CPU1_

	/*-------------------------------------------------------------------------------------------*/

	.CPU2.clear_sec : ALIGN(4)
	{
		LONG(ADDR(.CPU2.bss)); LONG(SIZEOF(.CPU2.bss));
		LONG(-1); LONG(-1);
	} > RODATA_CPU2_
	
	.CPU2.copy_sec : ALIGN(4)
	{
		LONG(LOADADDR(.CPU2.ramcode)); LONG(0 + ADDR(.CPU2.ramcode)); LONG(SIZEOF(.CPU2.ramcode));
		LONG(LOADADDR(.CPU2.data)); LONG(0 + ADDR(.CPU2.data)); LONG(SIZEOF(.CPU2.data));
		LONG(-1); LONG(-1); LONG(-1);
	} > RODATA_CPU2_

}


/* ================================================================================================
 * Linker Symbols
 * ==============================================================================================*/

/* Read only small address pointer */
_SMALL_DATA2_ = ADDR(.sdata2) + 0x8000;

/* Volatile data short address pointer (.sdata before .sbss) */
_SMALL_DATA_ = ADDR(.sdata) + 0x8000;

/* Expected HEAP SYMBOLS */
__HEAP     = ADDR(.heap);
__HEAP_END = SIZEOF(.heap);


/* ================================================================================================
 * CRT0 CONFIG STRUCTURE
 * Initialization structure for uC Cores used in crt0 startup code
 * Each core has itw one table entry.
 * ==============================================================================================*/

SECTIONS
{
	.crt0_config : ALIGN(4)
	{
		PROVIDE(__crt0_config = .);        /* base of the crt0 config table */
		
		/* Core[0] */
		LONG(ADDR(.CPU0.stack));           /* STACK address */ 
		LONG(SIZEOF(.CPU0.stack));         /* STACK size */
		LONG(ADDR(.CPU0.csa));             /* CSA address */ 
		LONG(SIZEOF(.CPU0.csa));           /* CSA size */
		LONG(_SMALL_DATA_);                /* SDATA address */
		LONG(_SMALL_DATA2_);               /* SDATA2 address */
		LONG(0);                           /* SDATA3 address */
		LONG(0);                           /* SDATA4 address */
		LONG(ADDR(.CPU0.clear_sec));       /* CLEAR table */
		LONG(ADDR(.CPU0.copy_sec));        /* COPY table */
		LONG(__CTOR_LIST__);               /* CTOR table */
		
		/* Core[1] */
		LONG(ADDR(.CPU1.stack));           /* STACK address */ 
		LONG(SIZEOF(.CPU1.stack));         /* STACK size */
		LONG(ADDR(.CPU1.csa));             /* CSA address */ 
		LONG(SIZEOF(.CPU1.csa));           /* CSA size */
		LONG(_SMALL_DATA_);                /* SDATA address */
		LONG(_SMALL_DATA2_);               /* SDATA2 address */
		LONG(0);                           /* SDATA3 address */
		LONG(0);                           /* SDATA4 address */
		LONG(ADDR(.CPU1.clear_sec) );      /* CLEAR table */
		LONG(ADDR(.CPU1.copy_sec));        /* COPY table */
		LONG(0);                           /* CTOR table */
		
		/* Core[2] */
		LONG(ADDR(.CPU2.stack));           /* STACK address */ 
		LONG(SIZEOF(.CPU2.stack));         /* STACK size */
		LONG(ADDR(.CPU2.csa));             /* CSA address */ 
		LONG(SIZEOF(.CPU2.csa));           /* CSA size */
		LONG(_SMALL_DATA_);                /* SDATA address */
		LONG(_SMALL_DATA2_);               /* SDATA2 address */
		LONG(0);                           /* SDATA3 address */
		LONG(0);                           /* SDATA4 address */
		LONG(ADDR(.CPU2.clear_sec));       /* CLEAR table */
		LONG(ADDR(.CPU2.copy_sec));        /* COPY table */
		LONG(0);                           /* CTOR table */

	} > RODATA 
}


/* ================================================================================================
 * DWARF debug sections and others
 * Symbols in the DWARF debugging sections are relative to the
 * beginning of the section, so we begin them at 0.
 * ==============================================================================================*/

SECTIONS
{	
	/* DWARF 1 */
	.comment            0 : { *(.comment) }
	.debug              0 : { *(.debug) }
	.line               0 : { *(.line) }
	
	/* GNU DWARF 1 extensions */
	.debug_srcinfo      0 : { *(.debug_srcinfo) }
	.debug_sfnames      0 : { *(.debug_sfnames) }
	
	/* DWARF 1.1 and DWARF 2 */
	.debug_aranges      0 : { *(.debug_aranges) }
	.debug_pubnames     0 : { *(.debug_pubnames) }
	
	/* DWARF 2 */
	.debug_info         0 : { *(.debug_info) }
	.debug_abbrev       0 : { *(.debug_abbrev) }
	.debug_line         0 : { *(.debug_line) }
	.debug_frame        0 : { *(.debug_frame) }
	.debug_str          0 : { *(.debug_str) }
	.debug_loc          0 : { *(.debug_loc) }
	.debug_macinfo      0 : { *(.debug_macinfo) }
	.debug_ranges       0 : { *(.debug_ranges) }
	
	/* DWARF 2 control flow extension */
	.debug_control_flow 0 : { *(.debug_control_flow) }
	
	/* SGI/MIPS DWARF 2 extensions */
	.debug_weaknames    0 : { *(.debug_weaknames) }
	.debug_funcnames    0 : { *(.debug_funcnames) }
	.debug_typenames    0 : { *(.debug_typenames) }
	.debug_varnames     0 : { *(.debug_varnames) }
	
	.version_info       0 : { *(.version_info) }
}
