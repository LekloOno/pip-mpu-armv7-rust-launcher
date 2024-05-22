use crate::pip_mpu::core::pip_core_mpu;
use crate::pip_mpu::core::pip_items::BlockOrError;
use crate::pip_mpu::core::pip_items::YieldCode;

/// Brief.
///     Creates a new child
///
/// Description.
///     The [createPartition] system call creates a new child (sub-partition of the
///     current partition), e.g. initializes the block `block_local_id` as a PD block and
///     sets the current partition as the parent partition.
///
/// *   `block_local_id` - The block to become the child partition descriptor
///
/// Returns
///     A Result such as in case of :
///         - Success   : Empty Ok()
///         - Error     : Empty Err()
pub fn create_partition(block_local_id: &*const u32) -> Result<(), ()> {
    if (pip_core_mpu::pip_create_partition(block_local_id) & 1) == 1 {
        Ok(())
    } else {
        Err(())
    }
}

/// Brief.
///     Cuts the given memory block.
///
/// Description.
///     The [cutMemoryBlock] system call cuts the memory block `block_to_cut_local_id`
/// 	at `cut_addr` which creates a new subbblock at that address.
///     The new subblock is placed in the physical MPU region of the current partition
///     if the `mpu_region_nb` is a valid region number.
///
/// *   `block_to_cut_local_id` - The local id of the block to cut
/// *   `cut_addr`              - The adress at which to create the new subblock
/// *   `mpu_region_nb`         - The mpu region number
///
/// Returns
///     A Result such as in case of :
///         - Success   : Ok() containing the newly created subblock's local id
///         - Error     : Empty Err()
pub fn cut_memory_block(
    block_to_cut_local_id: &*const u32,
    cut_addr: &*const u32,
    mpu_region_nb: i32,
) -> Result<*const u32, ()> {
    let subblock_local_id =
        pip_core_mpu::pip_cut_memory_block(block_to_cut_local_id, cut_addr, mpu_region_nb);
    subblock_local_id
        .is_null()
        .then(|| subblock_local_id)
        .ok_or(())
}

/// Brief.
///     Merge the given memory blocks to the given mpu region number.
///
/// Description.
///     The [mergeMemoryBlocks] system call merges `block_to_merge_1_local_id` and
///     `block_to_merge_2_local_id` together.
///     The two blocks have been cut before so @block_to_merge_1_local_id < @block_to_merge_2_local_id.
///     The merged block is placed in the physical MPU region of the current partition
///     if the `mpu_region_nb` is a valid region number.
///
/// *   `block_to_merge_1_local_id` - The local id of the first block to merge
/// *   `block_to_merge_2_local_id` - The local id of the second block to merge
/// *   `mpu_region_nb`             - The mpu regio number
///
/// Returns
///     A Result such as in case of :
///         - Success   : Ok() containing the newly created merged block's local id
///         - Error     : Empty Err()
pub fn merge_memory_blocks(
    block_to_merge_1_local_id: &*const u32,
    block_to_merge_2_local_id: &*const u32,
    mpu_region_nb: i32,
) -> Option<*const u32> {
    let merged_block_local_id = pip_core_mpu::pip_merge_memory_blocks(
        block_to_merge_1_local_id,
        block_to_merge_2_local_id,
        mpu_region_nb,
    );
    merged_block_local_id
        .is_null()
        .then(|| merged_block_local_id)
}

/// Brief.
///     Prepares the current partition or child to receive a projected_slots_nb of blocks
///     
/// Description.
///     The [prepare] system call prepares the partition of `part_desc_block_id`
///		(current partition or child) to receive `projected_slots_nb` of blocks and use the
///		`requisitionned_block_local_id` as a metadata structure IF NEEDED => if there isn't enough
///                                                                     free slots in the current
///                                                                     kernel structure.
///
///     e.g. this will prepare `requisitionned_block_local_id` to be a kernel structure added to the
///		kernel structure list of the partition `part_desc_block_id`
///        - if enough free slots to receive `projected_slots_nb` then won't do anything
///				- if not enough free slots then prepare the block
///        - if `projected_slots_nb` not specified then prepare the block whatever the nb of
///					free slots
///
/// *   `part_desc_block_id`            - The block to prepare within the current or child partition
/// *   `projected_slots_nb`            - The number of requested slots, do not specify to force prepare
/// *   `requisitionned_block_local_id` - The block used as the new kernel structure
///
/// Returns
///     A Result such as in case of :
///         - Valid Operation   : Empty Ok()
///         - Unvalid Operation : Empty Err()
///
///     TODO :
///     -   Ok(bool) - contains true if the requisitionned block was used, false otherwise
///     -   Err() - ..... Many cases to determine
/// ____
/// Note : Note satisfied with the 'None' slots nb. Maybe an enum ? Looking for better ideas
pub fn prepare(
    part_desc_block_id: &*const u32,
    projected_slots_nb: Option<i32>,
    requisitionned_block_local_id: &*const u32,
) -> Result<(), ()> {
    let valid = pip_core_mpu::pip_prepare(
        part_desc_block_id,
        projected_slots_nb.unwrap_or_else(|| -1),
        requisitionned_block_local_id,
    ) & 1
        == 1;

    valid.then(|| {}).ok_or(())
}

/// Brief.
///     Adds a memory block to a given child partition.
///     
/// Description.
///     The [addMemoryBlock] system call adds a block to a child partition.
///		The block is still accessible from the current partition (shared memory).
///
/// *   child_part_desc_block_local_id  - The local id of the child partition to share with
/// *   block_to_share_local_id         - The local id of the block entry address where the block to share lies
/// *   r                               - The reading rights to apply to the child partition
/// *   w                               - ----writing---------------------------------------
/// *   x                               - ----execute---------------------------------------
///
/// Returns
///     A Result such as in case of :
///         - Success : Ok() containing the local id of the block in the child. (newly "mapped" id)
///         - Error   : empty Err()
pub fn add_memory_block(
    child_part_desc_block_local_id: &*const u32,
    block_to_share_local_id: &*const u32,
    r: bool,
    w: bool,
    x: bool,
) -> Result<*const u32, ()> {
    let added_block_local_id = pip_core_mpu::pip_add_memory_block(
        child_part_desc_block_local_id,
        block_to_share_local_id,
        r as u32,
        w as u32,
        x as u32,
    );

    added_block_local_id
        .is_null()
        .then(|| added_block_local_id)
        .ok_or(())
}

/// Brief.
///
/// Description.
///     The [removeMemoryBlock] system call removes a block from a child partition.
///     
///     This operation succeeds for any shared memory block previously added, but
///		fails if the purpose of the block is not shared memory anymore,
///		in particular in such cases:
///        - The block can't be removed if the child or its descendants use it
///					(or part of it) as a kernel structure
///        - The block can't be removed if the child's descendants cut the block
///     An unnaccessible block can still be removed if it is cut and all its
///     subbblocks are still accessible, == "can be merged back together"
///
/// *   block_to_remove_local_id - The local id of the block entry address where the block to remove lies
///
/// Returns
///     A Result such as in case of :
///         - Success   : Empty Ok()
///         - Error     : Empty Err()
pub fn remove_memory_block(block_to_remove_local_id: &*const u32) -> Result<(), ()> {
    if pip_core_mpu::pip_remove_memory_block(block_to_remove_local_id) & 1 == 1 {
        Ok(())
    } else {
        Err(())
    }
}

/// Brief.
///     Deletes the specitfied partition from the current partition.
///
/// Description.
///     The [deletePartition] system call deletes the partition `child_part_desc_block_local_id`
///		which is a child of the current partition, e.g. prunes the partition tree by removing
///		all references of the child and its respective blocks from the current partition.
///
/// *   child_part_desc_block_local_id - The local id of the descriptor block of the child to delete
///
/// Returns
///     A Result such as in case of :
///         - Success   : Empty Ok()
///         - Error     : Empty Err()
///             Null adress
///             Not a child partition
pub fn delete_partition(child_part_desc_block_local_id: &*const u32) -> Result<(), ()> {
    if pip_core_mpu::pip_delete_partition(child_part_desc_block_local_id) & 1 == 1 {
        Ok(())
    } else {
        Err(())
    }
}

/// Brief.
///     Collects and empty structure from the given partition, child or current.
///
/// Description.
///     The [collect] system call collects an empty structure (if possible) from
///		the partition `part_desc_block_id` (current partition or a child) and
///		returns the retrieved block.
///
/// *   part_desc_block_id - The global or local id of the descriptor block of the current or child partition
///
/// Returns
///     A Result such as in case of :
///         - Success : Ok() containing the local id of collected structure block
///         - Error   : empty Err()
/// ____
/// Note: This function refers to collect from pip-core-mpu
/// see https://gitlab.univ-lille.fr/2xs/pip/pipcore-mpu/-/blob/master/src/core/Services.v?ref_type=heads#L665-703
pub fn collect(part_desc_block_id: &*const u32) -> Result<*const u32, ()> {
    let collected_block_local_id = pip_core_mpu::pip_collect(part_desc_block_id);

    collected_block_local_id
        .is_null()
        .then(|| collected_block_local_id)
        .ok_or(())
}

/// Brief.
///     Maps the given block of the given partition to the given MPU region.
///
/// Description.
///     The [mapMPU] system call maps the `block_to_map_local_id` block owned by
///		the partition `part_desc_block_id` (current partition or a child) in the
///     `mpu_region_nb` MPU region.
///		If the block is NULL, then the targeted MPU region is removed from the MPU.
///		If the block was already mapped, moves the block to the given MPU region.
///
/// *   part_desc_block_id      - The global or local id of the descriptor block of the current or child partition
/// *   block_to_map_local_id   - The block to map local id
/// *   mpu_region_nb           - The physical MPU region number
///
/// Returns
///     A Result such as in case of :
///         - Did map the given block   : Empty Ok()
///         - Other cases               : Empty Err()
///             - No block to map specified                             - block removed from the given region nb
///             - `block_to_map_local_id` is not accessible             - block removed from the given region nb
///             - `part_desc_block_id` not current nor child partition  - nothing
///             - `mpu_region_nb` is not a valid region number          - nothing
/// ____
/// Note: This function refers to mapMPU from pip-core-mpu
/// see https://gitlab.univ-lille.fr/2xs/pip/pipcore-mpu/-/blob/master/src/core/Services.v?ref_type=heads#L706-759
pub fn map_mpu(
    part_desc_block_id: &*const u32,
    block_to_map_local_id: &*const u32,
    mpu_region_nb: i32,
) -> Result<(), ()> {
    if pip_core_mpu::pip_map_mpu(part_desc_block_id, block_to_map_local_id, mpu_region_nb) & 1 == 1
    {
        Ok(())
    } else {
        Err(())
    }
}

/// Brief.
///     Reads the content of the given mpu region.
///
/// Description.
///     The [readMPU] system call reads the content of the physical MPU owned by
///		the partition `part_desc_block_id` (current partition or a child) at the
///     `mpu_region_nb` MPU region.
///
/// *   part_desc_block_id  - The global or local id of the descriptor block of the current or child partition
/// *   mpu_region_nb       - The physical MPU region number
///
/// Returns
///     A Result such as in case of :
///         - Success : Ok() containing the local id of the block to read
///         - Error   : empty Err()
///             - No block found or error
/// ____
/// Note: This function refers to readMPU from pip-core-mpu
/// see https://gitlab.univ-lille.fr/2xs/pip/pipcore-mpu/-/blob/master/src/core/Services.v?ref_type=heads#L762-794
pub fn read_mpu(part_desc_block_id: &*const u32, mpu_region_nb: i32) -> Result<*const u32, ()> {
    let block_read_local_id = pip_core_mpu::pip_read_mpu(part_desc_block_id, mpu_region_nb);

    block_read_local_id
        .is_null()
        .then(|| block_read_local_id)
        .ok_or(())
}

/// Brief.
///     Finds the block at the given source address in given partition block and insert it at the given target address.
///
/// Description.
///     The [findBlock] system call finds the block of the provided `addr_in_block`
///		by searching in the blocks list of the partition descriptor `part_desc_block_id`.
///     Writes the found block at the `target_block_addr`.
///
/// *   part_desc_block_id  - The global or local id of the descriptor block of the current or child partition
/// *   addr_in_block       - The address stemming from the block to find
/// *   target_block_addr   - The address where to write the found block's attributes
///
/// Returns
///     A Result such as in case of :
///         - Success   : Empty Ok()
///         - Error     : Empty Err()
///             `part_desc_block_id` is not a child nor current partition
///             `addr_in_block` not in partition of `part_desc_block_id`
///             `target_block_addr` not in partition of `part_desc_block_id`
/// ____
/// Note: This function refers to findBlock from pip-core-mpu
/// see https://gitlab.univ-lille.fr/2xs/pip/pipcore-mpu/-/blob/master/src/core/Services.v?ref_type=heads#L796-840
pub fn find_block(
    part_desc_block_id: &*const u32,
    addr_in_block: &*const u32,
    block_addr: &*const BlockOrError,
) -> Result<(), ()> {
    if pip_core_mpu::pip_find_block(part_desc_block_id, addr_in_block, block_addr) & 1 == 1 {
        Ok(())
    } else {
        Err(())
    }
}

/// Brief.
///     Sets the VIDT address in the given partition.
///     
/// Description.
///     The [setVIDT] system call sets the VIDT address in the partition
///     descriptor structure of the current partition or one of its child.
///
/// *   part_desc_block_id  -   The global or local id of the block containing
///                             the descriptor structure of the current or
///                             child partition
/// *   vidt_block_local_id -   The address of the VIDT or NULL to reset the
///                             VIDT address to NULL in the partition descriptor
///
/// Returns
///     A Result such as in case of :
///         - Success   : Empty Ok()
///         - Error     : Empty Err()
///             `part_desc_block_id` is not a partition
///             VIDT block is null
///             VIDT block is not present
///             VIDT block is not accessible
///             VIDT block overlaps
///             VIDT block is shared
///             VIDT block is shared in child
/// ____
/// Note: This function refers to setVIDT from pip-core-mpu
/// see https://gitlab.univ-lille.fr/2xs/pip/pipcore-mpu/-/blob/master/src/core/Services.v?ref_type=heads#L842-914
pub fn set_vidt(
    part_desc_block_id: &*const u32,
    vidt_block_local_id: &*const u32,
) -> Result<(), ()> {
    if pip_core_mpu::pip_set_vidt(part_desc_block_id, vidt_block_local_id) & 1 == 1 {
        Ok(())
    } else {
        Err(())
    }
}

/// Brief.
///    Yields from a the current partition to a callee.
///
/// Description.
///     The [yield] system call yields from the current partition (the caller)
///     to its parent, itself or one of its children (the callee).
///
/// *   callee_part_desc_block_id       -   The local id of the block containing the descriptor structure of the partition to yield to
///                                         0 means the partition descriptor structure of the parent of the current partition
/// *   user_target_interrupt           -   The index of the VIDT which contains the address pointing to the location where the current
///                                         context is to be RESTORED
/// *   user_caller_context_save_index  -   The index of the VIDT which contains the address pointing to the location where the current
///                                         is to be STORED
///                                         0 means the context is not stored
/// *   flags_on_yield                  -   The state the callee partition wishes to be on yield
/// *   flags_on_wake                   -   The state the caller partition wishes to be on wake
///
/// Returns
///     An Error code such as
///             BASICS
///              1  : The VIDT index of the CALLEE is greater than 32.
///              2  : ----------------------CALLER--------------------
///              3  : The CALLEE is not a child of the CALLER, although the given id is neither null or the current partition desc block.
///              4  : The root partition tried to call its parent.
///
///             CALLER's VIDT
///              5  : The address of the block containing the VIDT of the CALLER is null.
///              6  : The block containing the VIDT of the CALLER does not have the present flag.
///              7  : --------------------------------------------does not have the accessible flag.
///              8  : --------------------------------------------is too small.
///
///             CALLEE's VIDT
///              9  : The address of the block containing the VIDT of the CALLEE is null.
///             10  : The block containing the VIDT of the CALLEE does not have the present flag.
///             11  : --------------------------------------------does not have the accessible flag.
///             12  : --------------------------------------------is too small.
///
///             CALLER's context
///             13  : No block were found in the CALLER's address space that match the context address read from the VIDT.
///             14  : The block containing the address to which the context of the CALLER is to be written does not have the present flag.
///             15  : -------------------------------------------------------------------------------------------------------accessible flag.
///             16  : -------------------------------------------------------------------------------------------------------writable flag.
///             17  : The address of the CALLER's context, added to the size of a context, exceeds the end of the block.
///             18  : The address to which the CALLER's context should be written is not alligned on a 4-byte boundary.
///
///             CALLEE's context
///             19  : No block were found in the CALLEE's address space that match the context address read from the VIDT.
///             20  : The block containing the address to which the context of the CALLEE is to be written does not have the present flag.
///             21  : -------------------------------------------------------------------------------------------------------accessible flag.
///             22  : -------------------------------------------------------------------------------------------------------readable flag.
///             23  : The address of the CALLEE's context, added to the size of a context, exceeds the end of the block.
///             24  : The address at which the CALLEE's context should be read is not aligned on a 4-byte boundary.
///         Return value should be ignored when the context is restored.
/// ____
/// Note: This function refers to yield from pip-core-mpu
/// see https://gitlab.univ-lille.fr/2xs/pip/pipcore-mpu/-/blob/master/src/arch/dwm1001/boot/yield_c.c?ref_type=heads
pub fn r#yield(
    callee_part_desc_block_id: &*const u32,
    user_target_interrupt: u32,
    user_caller_context_save_index: u32,
    flags_on_yield: u32,
    flags_on_wake: u32,
) -> YieldCode {
    YieldCode::from_u32(pip_core_mpu::pip_yield(
        callee_part_desc_block_id,
        user_target_interrupt,
        user_caller_context_save_index,
        flags_on_yield,
        flags_on_wake,
    ))
    .unwrap()
}

/// Brief.
///     Gets the given partition interrupt state.
///
/// Description.
///     The [getIntState] system call gets the child partition of `child_part_desc_block_local_id` interrupt state.
///     Root partition can truly hide the interrupts, where as child partition vitually hides them, the root partition
///     should manage these interrupt states.
///
///     Reminder : Interrupts in pip-mpu flow down from pip, through root partition, down to the child partitions.
///     To manage child interrupt states, the root partition can check them with this system call, and do whatever should
///     be done accordingly.
///
/// *   child_part_desc_block_local_id   - The local id of the block containing the descriptor structure of the given child partition
///
/// Returns
///     True if the interruption are masked for this partition, false otherwise.
/// ____
/// Note: This function refers to getIntState from pip-core-mpu
/// see https://gitlab.univ-lille.fr/2xs/pip/pipcore-mpu/-/blob/master/src/arch/dwm1001/boot/pip_interrupt_calls.c?ref_type=heads#L40-54
pub fn child_has_hidden_int(child_part_desc_block_local_id: *const u32) -> bool {
    pip_core_mpu::pip_get_int_state(child_part_desc_block_local_id) & 1 == 1
}

/// Brief.
///     Gets the current partition interrupt state.
///
/// Description.
///     The [getSelfIntState] system call gets the current partition interrupt state.
///     Root partition can truly hide the interrupts, where as child partition vitually hides them, the root partition
///     should manage these interrupt states.
///
///     Reminder : Interrupts in pip-mpu flow down from pip, through root partition, down to the child partitions.
///     To manage child interrupt states, the root partition can check them with this system call, and do whatever should
///     be done accordingly.
///
/// Returns
///     True if the interruption are masked for this partition, false otherwise.
/// ____
/// Note: This function refers to getIntState from pip-core-mpu
/// see https://gitlab.univ-lille.fr/2xs/pip/pipcore-mpu/-/blob/master/src/arch/dwm1001/boot/pip_interrupt_calls.c?ref_type=heads#L56-64
pub fn self_has_hidden_int() -> bool {
    pip_core_mpu::pip_get_self_int_state() & 1 == 1
}
