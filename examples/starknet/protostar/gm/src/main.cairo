# SPDX-License-Identifier: MIT

%lang starknet


from starkware.cairo.common.cairo_builtins import HashBuiltin

@constructor
func constructor{
        syscall_ptr: felt*, 
        pedersen_ptr: HashBuiltin*,
        range_check_ptr
    }():
    return ()
end

@view
func gm{
        syscall_ptr : felt*,
        pedersen_ptr : HashBuiltin*,
        range_check_ptr
    }():
    return ()
end