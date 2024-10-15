#![no_std]
/**************************** BEGIN TYPE DEFINITIONS ********************************/
type slab_flags_t = u32;

#[repr(C)]
pub struct kmem_cache {
    //#[cfg(not(feature = "slub_tiny"))]
    //cpu_slab: *mut KmemCacheCpu, // Raw pointer to per-cpu slab

    //flags: SlabFlags,             // slab_flags_t equivalent
    min_partial: u64,             // unsigned long -> u64 or usize
    size: u32,                    // unsigned int -> u32
    object_size: u32,             // unsigned int -> u32
    //reciprocal_size: ReciprocalValue,
    offset: u32,                  // unsigned int -> u32

    #[cfg(feature = "slub_cpu_partial")]
    cpu_partial: u32,             // unsigned int -> u32
    #[cfg(feature = "slub_cpu_partial")]
    cpu_partial_slabs: u32,       // unsigned int -> u32

    //oo: KmemCacheOrderObjects,
    //min: KmemCacheOrderObjects,
    //allocflags: GfpFlags,         // gfp_t equivalent in Rust
    refcount: i32,                // int -> i32
    ctor: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,  // Function pointer
    inuse: u32,
    align: u32,
    red_left_pad: u32,

    name: *const core::ffi::c_char,  // const char * -> C-compatible string pointer
    //list: ListHead,                // struct list_head equivalent in Rust

    #[cfg(feature = "sysfs")]
    kobj: Kobject,                // struct kobject equivalent

    #[cfg(feature = "slab_freelist_hardened")]
    random: u64,                  // unsigned long -> u64 or usize

    #[cfg(feature = "numa")]
    remote_node_defrag_ratio: u32, // unsigned int -> u32

    #[cfg(feature = "slab_freelist_random")]
    random_seq: *mut u32,         // unsigned int * -> raw pointer to u32

    #[cfg(feature = "kasan_generic")]
    kasan_info: KasanCache,       // struct kasan_cache equivalent

    #[cfg(feature = "hardened_usercopy")]
    useroffset: u32,              // unsigned int -> u32
    #[cfg(feature = "hardened_usercopy")]
    usersize: u32,                // unsigned int -> u32

    //node: [*mut KmemCacheNode; MAX_NUMNODES], // Array of raw pointers to node structures
}

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
extern "C" {
    fn cache_line_size() -> u32;
    fn arch_slab_minalign() -> u32;
}
/*
 * Figure out what the alignment of the objects will be given a set of
 * flags, a user specified alignment and the size of the objects.
 */
#[no_mangle]
pub extern "C" fn calculate_alignment(flags: slab_flags_t,
    align: u32, size: u32) -> u32 {
    /*
    * If the user wants hardware cache aligned objects then follow that
    * suggestion if the object is sufficiently large.
    *
    * The hardware cache alignment cannot override the specified
    * alignment though. If that is greater then use it.
    */
    // Need to figure out what to do with all the stupid define flags
    //if flags & SLAB_HWCACHE_ALIGN {
    unsafe {
        let mut ralign = cache_line_size();
        while size <= ralign / 2 {
            ralign /= 2;
        }
        let mut align = core::cmp::max(align, ralign);
    //}

    align = core::cmp::max(align, arch_slab_minalign());
    align = align_macro(align, core::mem::size_of::<*const ()>() as u32);
    align
    }
}

// Ported function to Rust
#[no_mangle]
pub extern "C" fn kmem_cache_size(s: &kmem_cache) -> u32 {
    s.object_size
}