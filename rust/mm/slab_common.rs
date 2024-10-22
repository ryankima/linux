#![no_std]
/**************************** BEGIN STRUCT DEFINITIONS ********************************/

/**************************** BEGIN TYPE DEFINITIONS ********************************/
type slab_flags_t = u32;

// This comes from <linux/slab.h>
enum _slab_flag_bits {
	_SLAB_CONSISTENCY_CHECKS,
	_SLAB_RED_ZONE,
	_SLAB_POISON,
	_SLAB_KMALLOC,
	_SLAB_HWCACHE_ALIGN,
	_SLAB_CACHE_DMA,
	_SLAB_CACHE_DMA32,
	_SLAB_STORE_USER,
	_SLAB_PANIC,
	_SLAB_TYPESAFE_BY_RCU,
	_SLAB_TRACE,
	_SLAB_NOLEAKTRACE,
	_SLAB_NO_MERGE,
	_SLAB_ACCOUNT,
	_SLAB_NO_USER_FLAGS,
	_SLAB_SKIP_KFENCE,
	_SLAB_RECLAIM_ACCOUNT,
	_SLAB_OBJECT_POISON,
	_SLAB_CMPXCHG_DOUBLE,
	_SLAB_NO_OBJ_EXT,
	_SLAB_FLAGS_LAST_BIT
}

const SLAB_HWCACHE_ALIGN: u32 = 1 << (_slab_flag_bits::_SLAB_HWCACHE_ALIGN as u32);

/**************************** BEGIN MACRO DEFINITIONS ********************************/

// Equivalent to the __ALIGN_MASK macro
fn align_mask<T: Copy + core::ops::Add<Output = T> + core::ops::BitAnd<Output = T> + core::ops::Not<Output = T>>(x: T, mask: T) -> T {
    (x + mask) & !mask
}

// Equivalent to the ALIGN macro
fn align_macro<T: Copy + From<u8> + core::ops::Add<Output = T> + core::ops::BitAnd<Output = T> + core::ops::Not<Output = T> + core::ops::Sub<Output = T>>(x: T, a: T) -> T {
    let mask = a - T::from(1u8); // a - 1, same as in C
    align_mask(x, mask)
}

/**************************** BEGIN FUNCTION DEFINITIONS ********************************/

/*
 * Figure out what the alignment of the objects will be given a set of
 * flags, a user specified alignment and the size of the objects.
 */
#[no_mangle]
pub extern "C" fn calculate_alignment(flags: slab_flags_t,
    mut align: u32, size: u32) -> u32 {
    /*
    * If the user wants hardware cache aligned objects then follow that
    * suggestion if the object is sufficiently large.
    *
    * The hardware cache alignment cannot override the specified
    * alignment though. If that is greater then use it.
    */
    if flags & SLAB_HWCACHE_ALIGN != 0{
        let mut ralign = 128;
        while size <= ralign / 2 {
            ralign /= 2;
        }
        align = core::cmp::max(align, ralign);
    }

    align = core::cmp::max(align, core::mem::align_of::<u64>() as u32);
    align = align_macro(align, core::mem::size_of::<*const ()>() as u32);
    align
}

#[no_mangle]
pub extern "C" fn kmem_cache_size(s: &bindings::kmem_cache) -> u32 {
    //s.object_size
    0
}