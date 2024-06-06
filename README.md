Pip-mpu-armv7-rust-launcher includes various work on creating a rust environment
based on [Pip-MPU](https://gitlab.univ-lille.fr/2xs/pip/pipcore-mpu) kernel.

## Dependencies and toolchain installation

To build this project, you need:

1. a working rust environment with the nightly toolchain
   installed. Simplest solution is to follow the [Getting
   started](https://www.rust-lang.org/learn/get-started) and install
   `rustup`. Then do a simple: 
   ```bash
   rustup toolchain install nightly
   ``` 
    to add the nightly toolchain. This is needed as we are going to build
   `rust-std` from source for our target and use some unstable
   features

2. [cargo-make](https://crates.io/crates/cargo-make) that we using
   to perform all pre and post build operation to generate the final
   binary file that you will be able to link with Pip. You can install
   it using `cargo install cargo-make`

3. `arm-none-eabi-binutils` and `arm-none-eabi-gcc` as we are using
   `gcc` to perform link with specific options that are not currently
   supported by `lld`.

4. an environment in which the python modules listed in
   `relocation_tools/relocator/requirements.txt` are available.
   You can use a virtualenv to do have it
   ```bash
   python3 -m venv pip_venv
   source pip_venv/bin/activate
   pip install -r relocation_tools/relocator/requirements.txt
   ```
   
   
## Building project

To build the project, issue the following command

```bash
cargo make [-p release]
```

The optional `-p release` is used to select a release build profile
and thus build the code with all optimization and link time
optimization.

The final partition binary file will be located at the root of the
crate and be named `partition-dev.bin` or `partition-release.bin`
depending on the build profile used.


## About Manage Partition Module Structure

```
+-----+                         +-----+                         +-----+
|/////| pip reserved block      |\ \ \| parent reserved block   |     | child block
+-----+                         +-----+                         +-----+

RAM

Parent ram
/!\ This is an arbitrary representation /!\
#parent ram might be separated in multiple and non contiguous blocks
#the optional pip block could be before child_ram_block or in a completely different block
+-----------------------------+
|\ \ \ \ \ \ \ \ \ \ \ \ \ \ \|
| \ \ \ \ \ \ \ \ \ \ \ \ \ \ |               child_ram_block
+-----------------------------+_ _          +---------------------------------+
|\ \ \ \ \ \ \ \ \ \ \ \ \ \ \|             | \ \<optional> ram_head_block \ \| -> |Resultant of stack vidt alignment
|                             |   |         +---------------------------------+
|                             |             |        stack_vidt_block         |
|                             |   |         +---------------------------------+
|                             |         |   |         ctx_itf_block           |
|       child_ram_block       |   |---> |   +---------------------------------+
|                             |         |   |                                 |    |Could be no unused_ram_block if
|                             |   |         |   <optional> unused_ram_block   | -> |the if the given child_ram_block
|                             |             |                                 |    |just fits the stack_vidt & ctx_itf
|                             |   |         +---------------------------------+
|                             |_ _          |//////<optional> pip_block///////| -> |If no pip block is given to create
+-----------------------------+             +---------------------------------+    |the partition
|\ \ \ \ \ \ \ \ \ \ \ \ \ \ \|
| \ \ \ \ \ \ \ \ \ \ \ \ \ \ |
|\ \ \ \ \ \ \ \ \ \ \ \ \ \ \|             pip_block
+-----------------------------+_ _          +---------------------------------+
|/////////////////////////////|         |   |///////////pd_block_id///////////|
|////<optional> pip_block/////|   |---> |   +---------------------------------+ -> |pd and kernel structure of the
|/////////////////////////////|_ _      |   |//////////kern_block_id//////////|    |child partition
+-----------------------------+             +---------------------------------+
|\ \ \ \ \ \ \ \ \ \ \ \ \ \ \|
| \ \ \ \ \ \ \ \ \ \ \ \ \ \ |
+-----------------------------+
|//<optional> new_kern_block//| -> |If creating a new kernel structure is required
+-----------------------------+    |to prepare the new partition's blocks

___________________________________________________________________________________
ROM

Parent rom
(arbitrary representation, parent rom might be separated in multiple and non contiguous blocks)
+-----------------------------+                     +-----------------------------+
|\ \ \ \ \ \ \ \ \ \ \ \ \ \ \|                     |\ \ \ \ \ \ \ \ \ \ \ \ \ \ \|
| \ \ \ \ rom block X \ \ \ \ |                     | \ \ \ \ \ \ \ \ \ \ \ \ \ \ |
|\ \ \ \ \ \ \ \ \ \ \ \ \ \ \|                     |\ \ \ \ \ \ \ \ \ \ \ \ \ \ \|
+-----------------------------+                     +-----------------------------+
|\ \ \ \ \ \ \ \ \ \ \ \ \ \ \|                     |\ \ \ \ \ \ \ \ \ \ \ \ \ \ \|
| \ \ \ \ \ \ \ \ \ \ \ \ \ \ |                     | \<optional> rom_head_block\ |
|\ \ \ \ \ \ \ \ \ \ \ \ \ \ \|                     |\ \ \ \ \ \ \ \ \ \ \ \ \ \ \|
| \ \ \ \ \ \ \ \ \ \ \ \ \ \ | <-- entry_point --> +-----------------------------+
|\ \ \ \ \ \ \ \ \ \ \ \ \ \ \|         |           |                             |
| \ \ \ \ \ \ \ \ \ \ \ \ \ \ |    used_rom_size    |          rom_block          |
|\ \ \ \ \ \ \ \ \ \ \ \ \ \ \|         |           |                             |
| \ \ \ \ \ \ \ \ \ \ \ \ \ \ |       --+--         +-----------------------------+
|\ \ \ \ \rom block Y\ \ \ \ \|         |           |                             |
| \ \ \ \ \ \ \ \ \ \ \ \ \ \ |   unused_rom_size   | <optional> unused_rom_block |
|\ \ \ \ \ \ \ \ \ \ \ \ \ \ \|         |           |                             |
| \ \ \ \ \ \ \ \ \ \ \ \ \ \ |       --+--         +-----------------------------+
|\ \ \ \ \ \ \ \ \ \ \ \ \ \ \|                     |\ \ \ \ \ \ \ \ \ \ \ \ \ \ \|
| \ \ \ \ \ \ \ \ \ \ \ \ \ \ |                     | \<optional> rom_tail_block\ |
|\ \ \ \ \ \ \ \ \ \ \ \ \ \ \|                     |\ \ \ \ \ \ \ \ \ \ \ \ \ \ \|
+-----------------------------+                     +-----------------------------+
| \ \ \ \ \ \ \ \ \ \ \ \ \ \ |                     | \ \ \ \ \ \ \ \ \ \ \ \ \ \ |
|\ \ \ \ \rom block Z\ \ \ \ \|                     |\ \ \ \ \ \ \ \ \ \ \ \ \ \ \|
| \ \ \ \ \ \ \ \ \ \ \ \ \ \ |                     | \ \ \ \ \ \ \ \ \ \ \ \ \ \ |
+-----------------------------+                     +-----------------------------+
```

When creating a partition, you will receive a `CreateReturn`.

## `CreateReturn`
|Property|Type|Description|
|--------|----|-----------|
|partition|`Partition`|Informations about the created Partition's blocks.|
|parent_infos|`Parent`|Informations about the Parent's blocks.|
  
## `Partition`  
The exact meaning of this type can vary.
- If it is a `Partition` structure within a `Parent` structure, the Block ids are the local ids within the parent.
- If it is a `Partition` structure within a `CreateReturn` structure, the Block ids are the local ids within the child. 
 
|Property|Type|Description|
|--------|----|-----------|
|stack_vidt_block_id|`BlockId`|Local id of the created Partition's stack + vidt block.|
|ctx_itf_block_id|`BlockId`|Local id of the created Partition's context + interface block.|
|rom_block_id|`BlockId`|Local id of the created Partition's rom block.|
|unused_ram_block_id|`Option<BlockId>`|Local id of the created Partition's unused ram bloc, might be `None` if there was no left ram in the block given to create this partition.|
|unused_rom_block_id|`Option<BlockId>`|Local id of the created Partition's unused rom bloc, might be `None` if no unused rom space was requested.|

## `Parent`
|Property|Type|Description|
|--------|----|-----------|
|child_in_parent|`Partition`|The partition's blocks local ids within the parent.|
|ram_head_block_id|`Option<BlockId>`|Left over block resulting in the stack+vidt block alignment. `None` if the stack+vidt block could be aligned on the start address of the given child ram block.|
|rom_head_block_id|`Option<BlockId>`|Left over block resulting of the cut at the given entry point. The block which precedes the child's rom block. `None` if the entry point is the start address of a block.|
|rom_tail_block_id|`Option<BlockId>`|Left over block which succeeds the child's rom block. `None` if the requested rom size just fits the block containing the entry point address.|
|new_kern_block_id|`Option<BlockId>`|Newly created kernel structure to prepare the child's blocks. `None` if no new kernel structure was required.|
|pd_block_id|`BlockId`|Block local id of the child's partition descriptor. Will only be used to merge the child's blocks back when deleting the partition.|
|kern_block_id|`BlockId`|Block local id of the child's initial kernel structure. Will only be used to merge the child's block back when deleting the partition.|


### To do

- Check rom address is indeed rom.
- new kernel block, check if required, maybe create multiple ones.