#![no_std]

use core::ffi::{c_void, c_char};
use core::marker::Copy;
use core::convert::From;

use bindings;

/**************************** BEGIN DEFINES DEFINITIONS ********************************/
const CONFIG_64BIT: bool = true;
const system_has_cmpxchg128: bool = true;
const CONFIG_SLUB_CPU_PARTIAL: bool = true;
const system_has_freelist_aba: bool = true;
const CONFIG_SLAB_OBJ_EXT: bool = true;
const CONFIG_SLUB_TINY: bool = false;
const CONFIG_SYSFS: bool = true;
const CONFIG_SLAB_FREELIST_HARDENED: bool = true;
const CONFIG_NUMA: bool = true;
const CONFIG_SLAB_FREELIST_RANDOM: bool = true;
const CONFIG_HARDENED_USERCOPY: bool = true;
const SLAB_SUPPORTS_SYSFS: bool = true;
const CONFIG_SLUB_DEBUG: bool = true;
const CONFIG_PRINTK: bool = true;
const MAX_NUMNODES: u32 = 16;

/**************************** BEGIN TYPE DEFINITIONS ********************************/
#[allow(non_camel_case_types)]
type slab_flags_t = u32;

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum Gfp {
    GFP_KERNEL = 0,
    GFP_ATOMIC = 1,
    __GFP_HIGHMEM = 2,
    __GFP_HIGH = 3,
}

#[allow(non_camel_case_types)]
type gfp_t = Gfp;

/**************************** BEGIN STRUCT DEFINITIONS ********************************/

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct reciprocal_value {
    m: u32,
    sh1: u8,
    sh2: u8
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct kmem_cache_order_objects {
    x: u32
}


#[repr(C)]
#[allow(non_camel_case_types)]
struct kmem_cache_node {
    list_lock: bindings::spinlock_t,
    nr_partial: u64,
    partial: bindings::list_head
    /*
    #ifdef CONFIG_SLUB_DEBUG
        atomic_long_t nr_slabs;
        atomic_long_t total_objects;
        struct list_head full;
    #endif
    */
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct kmem_cache {
    //TODO: this __percpu is weird
    //struct kmem_cache_cpu __percpu *cpu_slab,
    /* Used for retrieving partial slabs, etc. */
    flags: slab_flags_t,
    min_partial: u64,
    size: u32,		/* Object size including metadata */
    object_size: u32,	/* Object size without metadata */

    reciprocal_size: reciprocal_value,
    offset: u32,		/* Free pointer offset */
    /* Number of per cpu partial objects to keep around */
    cpu_partial: u32,
    /* Number of per cpu partial slabs to keep around */
    cpu_partial_slabs: u32,
    //TODO
    oo: kmem_cache_order_objects,
    /* Allocation and freeing of slabs */
    //TODO
    min: kmem_cache_order_objects,
    allocflags: gfp_t,		/* gfp flags to use on each alloc */
    refcount: i32,			/* Refcount for slab cache destroy */
    ctor: *mut c_void,	/* Object constructor */
    inuse: u32,		/* Offset to metadata */
    align: u32,		/* Alignment */
    red_left_pad: u32,	/* Left redzone padding size */
    name: *const c_char,		/* Name (only for display!) */

    //TODO
    list: bindings::list_head,  /* List of slab caches */
    kobj: bindings::kobject,    /* For sysfs */
    random: u64,

    /*
        * Defragmentation by allocating from a remote node.
        */
    remote_node_defrag_ratio: u32,

    random_seq: *mut u32,
    
    useroffset: u32,	/* Usercopy region offset */
    usersize: u32,		/* Usercopy region size */

    //TODO
    //struct kmem_cache_node *node[MAX_NUMNODES];
    node: [*mut kmem_cache_node; MAX_NUMNODES],
}



// This comes from <linux/slab.h>
#[allow(non_camel_case_types)]
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

const slab_nomerge: bool = false; // TODO: check if this is actually false
const SLAB_HWCACHE_ALIGN: u32 = 1 << (_slab_flag_bits::_SLAB_HWCACHE_ALIGN as u32);
const SLAB_RED_ZONE: u32 = 1 << (_slab_flag_bits::_SLAB_RED_ZONE as u32);
const SLAB_POISON: u32 = 1 << (_slab_flag_bits::_SLAB_POISON as u32);
const SLAB_NO_USER_FLAGS: u32 = 1 << (_slab_flag_bits::_SLAB_NO_USER_FLAGS as u32);
const SLAB_STORE_USER: u32 = 1 << (_slab_flag_bits::_SLAB_STORE_USER as u32);
const SLAB_TRACE: u32 = 1 << (_slab_flag_bits::_SLAB_TRACE as u32);
const SLAB_TYPESAFE_BY_RCU: u32 = 1 << (_slab_flag_bits::_SLAB_TYPESAFE_BY_RCU as u32);
const SLAB_NOLEAKTRACE: u32 = 1 << (_slab_flag_bits::_SLAB_NOLEAKTRACE as u32);
const SLAB_NO_MERGE: u32 = 1 << (_slab_flag_bits::_SLAB_NO_MERGE as u32);

const SLAB_NEVER_MERGE: u32 = SLAB_RED_ZONE | SLAB_POISON | SLAB_STORE_USER |
                                SLAB_TRACE | SLAB_TYPESAFE_BY_RCU | SLAB_NOLEAKTRACE |
                                 SLAB_NO_MERGE;

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
    fn kmem_cache_flags(flags: slab_flags_t, name: *const c_char) -> slab_flags_t;
}


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

/*
 * Find a mergeable slab cache
 */
 #[no_mangle]
 pub extern "C" fn slab_unmergeable(s: *const kmem_cache) -> u32
 {
    unsafe {
        if slab_nomerge || ((*s).flags & SLAB_NEVER_MERGE != 0) {
            return 1;
        }
        if (*s).ctor != core::ptr::null_mut() {
            return 1;
        }
    
        if CONFIG_HARDENED_USERCOPY {
            if (*s).usersize != 0 {
                return 1;
            }
        }
        /*
        * We may have set a slab to be unmergeable during bootstrap.
        */
        if (*s).refcount < 0 {
            return 1;
        }
    }
    return 0;
 }

#[no_mangle]
pub extern "C" fn find_mergeable(size: u32, mut align: u32, mut flags: slab_flags_t,
                                 name: *const c_char, ctor: *const c_void) -> *mut kmem_cache
{
    //struct kmem_cache *s;

    if slab_nomerge {
        return core::ptr::null_mut();
    }

    if ctor != core::ptr::null() {
        return core::ptr::null_mut();
    }

    let mut size = align_macro(size,  core::mem::size_of::<*const ()>() as u32);
    align = calculate_alignment(flags, align, size);
    size = align_macro(size, align);
    unsafe {
        flags = kmem_cache_flags(flags, name);
    }
    if flags & SLAB_NEVER_MERGE != 0 {
        return core::ptr::null_mut();
    }
/*
list_for_each_entry_reverse(s, &slab_caches, list) {
    if (slab_unmergeable(s))
        continue;

    if (size > s->size)
        continue;

    if ((flags & SLAB_MERGE_SAME) != (s->flags & SLAB_MERGE_SAME))
        continue;
    /*
     * Check if alignment is compatible.
     * Courtesy of Adrian Drzewiecki
     */
    if ((s->size & ~(align - 1)) != s->size)
        continue;

    if (s->size - size >= sizeof(void *))
        continue;

    return s;
}
*/
    return core::ptr::null_mut();
}

/*
fn struct kmem_cache *create_cache(name: *const c_char,
    object_size: u32, align: u32,
    flags: slab_flags_t, useroffset: u32,
    usersize: u32, ctor: *mut c_void,
    root_cache: *const kmem_cache) -> *mut kmem_cache
{
    let mut s: ;
let mut err: u32;

if (WARN_ON(useroffset + usersize > object_size))
    useroffset = usersize = 0;

err = -ENOMEM;
s = kmem_cache_zalloc(kmem_cache, GFP_KERNEL);
if (!s)
    goto out;

s->name = name;
s->size = s->object_size = object_size;
s->align = align;
s->ctor = ctor;
#ifdef CONFIG_HARDENED_USERCOPY
s->useroffset = useroffset;
s->usersize = usersize;
#endif

err = __kmem_cache_create(s, flags);
if (err)
    goto out_free_cache;

s->refcount = 1;
list_add(&s->list, &slab_caches);
return s;

out_free_cache:
kmem_cache_free(kmem_cache, s);
out:
return ERR_PTR(err);
}
*/
/**
 * kmem_cache_create_usercopy - Create a cache with a region suitable
 * for copying to userspace
 * @name: A string which is used in /proc/slabinfo to identify this cache.
 * @size: The size of objects to be created in this cache.
 * @align: The required alignment for the objects.
 * @flags: SLAB flags
 * @useroffset: Usercopy region offset
 * @usersize: Usercopy region size
 * @ctor: A constructor for the objects.
 *
 * Cannot be called within a interrupt, but can be interrupted.
 * The @ctor is run when new pages are allocated by the cache.
 *
 * The flags are
 *
 * %SLAB_POISON - Poison the slab with a known test pattern (a5a5a5a5)
 * to catch references to uninitialised memory.
 *
 * %SLAB_RED_ZONE - Insert `Red` zones around the allocated memory to check
 * for buffer overruns.
 *
 * %SLAB_HWCACHE_ALIGN - Align the objects in this cache to a hardware
 * cacheline.  This can be beneficial if you're counting cycles as closely
 * as davem.
 *
 * Return: a pointer to the cache on success, NULL on failure.
 */
 /*
 struct kmem_cache *
 kmem_cache_create_usercopy(const char *name,
           unsigned int size, unsigned int align,
           slab_flags_t flags,
           unsigned int useroffset, unsigned int usersize,
           void (*ctor)(void *))
 {
     struct kmem_cache *s = NULL;
     const char *cache_name;
     int err;
 
 #ifdef CONFIG_SLUB_DEBUG
     /*
      * If no slab_debug was enabled globally, the static key is not yet
      * enabled by setup_slub_debug(). Enable it if the cache is being
      * created with any of the debugging flags passed explicitly.
      * It's also possible that this is the first cache created with
      * SLAB_STORE_USER and we should init stack_depot for it.
      */
     if (flags & SLAB_DEBUG_FLAGS)
         static_branch_enable(&slub_debug_enabled);
     if (flags & SLAB_STORE_USER)
         stack_depot_init();
 #endif
 
     mutex_lock(&slab_mutex);
 
     err = kmem_cache_sanity_check(name, size);
     if (err) {
         goto out_unlock;
     }
 
     /* Refuse requests with allocator specific flags */
     if (flags & ~SLAB_FLAGS_PERMITTED) {
         err = -EINVAL;
         goto out_unlock;
     }
 
     /*
      * Some allocators will constraint the set of valid flags to a subset
      * of all flags. We expect them to define CACHE_CREATE_MASK in this
      * case, and we'll just provide them with a sanitized version of the
      * passed flags.
      */
     flags &= CACHE_CREATE_MASK;
 
     /* Fail closed on bad usersize of useroffset values. */
     if (!IS_ENABLED(CONFIG_HARDENED_USERCOPY) ||
         WARN_ON(!usersize && useroffset) ||
         WARN_ON(size < usersize || size - usersize < useroffset))
         usersize = useroffset = 0;
 
     if (!usersize)
         s = __kmem_cache_alias(name, size, align, flags, ctor);
     if (s)
         goto out_unlock;
 
     cache_name = kstrdup_const(name, GFP_KERNEL);
     if (!cache_name) {
         err = -ENOMEM;
         goto out_unlock;
     }
 
     s = create_cache(cache_name, size,
              calculate_alignment(flags, align, size),
              flags, useroffset, usersize, ctor, NULL);
     if (IS_ERR(s)) {
         err = PTR_ERR(s);
         kfree_const(cache_name);
     }
 
 out_unlock:
     mutex_unlock(&slab_mutex);
 
     if (err) {
         if (flags & SLAB_PANIC)
             panic("%s: Failed to create slab '%s'. Error %d\n",
                 __func__, name, err);
         else {
             pr_warn("%s(%s) failed with error %d\n",
                 __func__, name, err);
             dump_stack();
         }
         return NULL;
     }
     return s;
 }
*/
/**
 * kfree_sensitive - Clear sensitive information in memory before freeing
 * @p: object to free memory of
 *
 * The memory of the object @p points to is zeroed before freed.
 * If @p is %NULL, kfree_sensitive() does nothing.
 *
 * Note: this function zeroes the whole allocated buffer which can be a good
 * deal bigger than the requested buffer size passed to kmalloc(). So be
 * careful when using this function in performance sensitive code.
 */
 /*
 void kfree_sensitive(const void *p)
 {
     size_t ks;
     void *mem = (void *)p;
 
     ks = ksize(mem);
     if (ks) {
         kasan_unpoison_range(mem, ks);
         memzero_explicit(mem, ks);
     }
     kfree(mem);
 }*/

#[no_mangle]
pub extern "C" fn  ksize(objp: *const c_void) -> u64
{
	/*
	 * We need to first check that the pointer to the object is valid.
	 * The KASAN report printed from ksize() is more useful, then when
	 * it's printed later when the behaviour could be undefined due to
	 * a potential use-after-free or double-free.
	 *
	 * We use kasan_check_byte(), which is supported for the hardware
	 * tag-based KASAN mode, unlike kasan_check_read/write().
	 *
	 * If the pointed to memory is invalid, we return 0 to avoid users of
	 * ksize() writing to and potentially corrupting the memory region.
	 *
	 * We want to perform the check before __ksize(), to avoid potentially
	 * crashing in __ksize() due to accessing invalid metadata.
	 */
     /*
	if (unlikely(ZERO_OR_NULL_PTR(objp)) || !kasan_check_byte(objp))
		return 0;

	return kfence_ksize(objp) ?: __ksize(objp);
    */
    0
}

#[no_mangle]
pub extern "C" fn kmem_cache_size(s: *const kmem_cache) -> u32 {
    unsafe {
        (*s).object_size
    }
}