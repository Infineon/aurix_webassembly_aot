## Variables
All are fields of struct Translator
### `cfg_label_stack`: 
- A stack of all control flow blocks (`block`, `if`, `loop`)
- Each element will be popped once you exit the block.
- The labels are numbers that identify each block, in the order they were created (0, 1, 2, ...)
- Either `Block` (includes loop) or `If`
- The `If` variant has a label for the end like the other and a label for the else 
### `cfg_block_result_stack`: 
- Each control flow block has its type (annotated in wasm)
- It is converted to a number of bytes for Tricore. 
- wasm stack after block = wasm stack before + result on top
- In wasm 1.0, no value is consumed and there is at most 1 result value.
- A (wasm stack) state is (stack size after block, Option\<block result size in bytes\>)
- Each `block_result` stores two state:
    - `end_state`: stack state after you arrive at block end.
    - `label_state`: stack state after you `break` to label, same as `end_state` for normal blocks, but no result for a `loop`

### `cfg_label_map`
- Stores the position in the instruction instruction corresponding to a label when you need to jump

### `cfg_jobs`
- For every jump instruction we store a job to replace the placeholder label with the actual target address once we know it
- The job is just the adress of the instruction in the instruction vector
- The placeholder we use is the index of the label in `cfg_label_map`
- Since the target address has been corrected in `cfg_label_map` we just need to replace the placeholder by the value in the map

### `dead_code_flag_stack`
- A stack of bools to keep track of nested blocks and set dead code flag for the current block
- If you enter a block that is dead code, all nested blocks are also dead code
- Some instructions like `br` will set the dead code flag for the current block
- When exiting a block, we pop the flag from the stack
- If code is dead, no Tricore code will be generated until we exit the block

### `vb_stack_ptr_stack` 
- Points to the top of the VB stack at the beginning of each cfg block to allow reset

## Entering a block
- Check if the block is dead code (if you were already in a dead code area before). If so, no code will be generated until we exit the block.
- Get the size of the block result from the blocktype (given by wasmparser).
- If it has a result you need to resolve all the VBs so that once the block is finished you can safely put the result on the stack.
- Compute the block result (depends on if it's a `loop` or not)
- Put the block result on the block result stack

### Label Management
- We push the new label with a new index (just the last index + 1) to `cfg_label_stack`
- For backward jumps (`loop`) the target is the position when entering the block
- For forward jumps target is unknown, we use a placeholder until block end
- We either add the target or a placeholder to `cfg_label_map`


## Entering an `if/else`
Enter the `if` block like a normal block, but skip it with a `JEQ` if the condition is false.
Replace the normal block label by an `If` block label so that it includes the else and end label.

The `else_label` will be `end_label` + 1
You also need to add a placeholder for the `else_label` in `cfg_label_map`

Then once you reach the `else` you replace the `If` block label by a normal block label with only the end label 

Then there are some things with the deadcode flags, because you need one for inside the if block and one for outside that will also include the else block

## Exiting
- Pop the dead code flag from the dead code flag stack
- Get the block result from `cfg_block_result_flag_stack`
- Pop the label from `cfg_label_stack`
- Resolve the VB at the top of the VB stack and put it in a register
- Adjust the stack pointer according to the block result (and its size)
- Update the label map for the popped label with the current instruction position
- Update the label of the block with the current instruction position

## Breaking to a label
- Get the label from the stack (counting from the top)
- Get the block result for that label (we only need the `label_state`)
- Resolve the VB the top of the VB stack and put it in a register
- Adjust the stack pointer according to the block result (and its size)
- Generate a jump instruction to the label
- Set the dead code flag for the current block to true
- Reset the VB stack to the pointer in `vb_stack_ptr_stack`

### Break if
- Generate a `JEQ` to skip the VB resolution if the condition is false
- If the condition is true, we resolve the VB and jump to the label
- The `JEQ` will jump to the instruction after the jump to break label
The code will look like this:
```
    JEQ skip_resolve
    resolve VB
    J break_label
skip_resolve:
    next instructions
    ...
break_label: (positioned after for a forward jump, could also be backward)
```
### Break table
- Resolve the index from the stack
- Generate a `JI` instruction that will jump to the index in a jump table
- The jump table will then bring you to a temporary label where you will do the VB resolution and jump to the actual break label

```
    JI index
    J temp_label_0
    J temp_label_1
    ...
    J temp_label_n
temp_label_0:
    resolve VB
    J break_label_0
temp_label_1:
    ...
temp_label_n:
    ...
break_label_0: (positioned after for a forward jump, could also be backward)
    some code
break_label_1: (positioned after for a forward jump, could also be backward)
    ...
```
- Set the dead code flag for the current block to true
- Reset the VB stack to the pointer in `vb_stack_ptr_stack`

## Replacing placeholder labels
- At the end of the Code section of the module, we replace all the placeholder labels in the jump instructions with the actual target addresses.