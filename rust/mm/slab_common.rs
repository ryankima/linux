#![no_std]

use core::ffi::{c_void, c_char, CStr};
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
const MAX_NUMNODES: usize = 16;
const KS_ADDRS_COUNT: usize = 16;

const EPERM: u32 =		 1;	/* Operation not permitted */
const ENOENT: u32 =		 2;	/* No such file or directory */
const ESRCH: u32 =		 3;	/* No such process */
const EINTR: u32 =		 4;	/* Interrupted system call */
const EIO: u32 =		 5;	/* I/O error */
const ENXIO: u32 =		 6;	/* No such device or address */
const E2BIG: u32 =	     7;	/* Argument list too long */
const ENOEXEC: u32 =	 8;	/* Exec format error */
const EBADF: u32 =		 9;	/* Bad file number */
const ECHILD: u32 =		10;	/* No child processes */
const EAGAIN: u32 =		11;	/* Try again */
const ENOMEM: u32 =		12;	/* Out of memory */
const EACCES: u32 =		13;	/* Permission denied */
const EFAULT: u32 =		14;	/* Bad address */
const ENOTBLK: u32 =	15;	/* Block device required */
const EBUSY: u32 =		16;	/* Device or resource busy */
const EEXIST: u32 =		17;	/* File exists */
const EXDEV: u32 =		18;	/* Cross-device link */
const ENODEV: u32 =		19;	/* No such device */
const ENOTDIR: u32 =	20;	/* Not a directory */
const EISDIR: u32 =		21;	/* Is a directory */
const EINVAL: u32 =		22;	/* Invalid argument */
const ENFILE: u32 =		23;	/* File table overflow */
const EMFILE: u32 =		24;	/* Too many open files */
const ENOTTY: u32 =		25;	/* Not a typewriter */
const ETXTBSY: u32 =	26;	/* Text file busy */
const EFBIG: u32 =		27;	/* File too large */
const ENOSPC: u32 =		28;	/* No space left on device */
const ESPIPE: u32 =		29;	/* Illegal seek */
const EROFS: u32 =		30;	/* Read-only file system */
const EMLINK: u32 =		31;	/* Too many links */
const EPIPE: u32 =		32;	/* Broken pipe */
const EDOM: u32 =		33;	/* Math argument out of domain of func */
const ERANGE: u32 =		34;	/* Math result not representable */

/**************************** BEGIN TYPE DEFINITIONS ********************************/
#[allow(non_camel_case_types)]
type slab_flags_t = u32;
type freelist_full_t = u64;

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
pub struct kmem_cache_node {
    list_lock: bindings::spinlock_t,
    nr_partial: u64,
    partial: bindings::list_head,

    nr_slabs: bindings::atomic_long_t,
    total_objects: bindings::atomic_long_t,
    full: bindings::list_head,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct kmem_cache_cpu {
    _unused: [u8; 0],
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct kmem_cache {
    //TODO: this __percpu is weird
    //struct kmem_cache_cpu __percpu *cpu_slab,
    cpu_slab: *mut kmem_cache_cpu,
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
    oo: kmem_cache_order_objects,
    /* Allocation and freeing of slabs */
    min: kmem_cache_order_objects,
    allocflags: gfp_t,		/* gfp flags to use on each alloc */
    refcount: i32,			/* Refcount for slab cache destroy */
    ctor: *mut c_void,	/* Object constructor */
    inuse: u32,		/* Offset to metadata */
    align: u32,		/* Alignment */
    red_left_pad: u32,	/* Left redzone padding size */
    name: *const c_char,		/* Name (only for display!) */

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


    node: [*mut kmem_cache_node; MAX_NUMNODES],
}


//Interior components of slab, previously were anonymous components
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub struct slab_member_numbered_list {
    next: *mut slab,
    slabs: i32, /* Nr of slabs left */
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub union slab_member_list_type {
    slab_list: bindings::list_head,
    slab_list_numbered: slab_member_numbered_list
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub struct slab_freelist_counter {
    freelist: *mut c_void,
    counters: u64,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub union freelist_aba_t {
    list: slab_freelist_counter,
    full: freelist_full_t
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub union slab_freelist_counter_type {
    freelist: slab_freelist_counter,
    freelist_aba: freelist_aba_t
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub struct slab_list_and_counter {
    list: slab_member_list_type,
    freelist_counter: slab_freelist_counter_type

}

//End interior slab components
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub struct slab {
    __page_flags: u64,
    slab_cache: *mut kmem_cache,
    slab_list: slab_list_and_counter,   //This is a bunch of nested structs and unions, it might be wrong
    
    __page_type: u32,
    __page_refcount: bindings::atomic_t,
    obj_exts: u64,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct slabinfo {
    active_objs: u64,
    num_objs: u64,
    active_slabs: u64,
    num_slabs: u64,
    shared_avail: u64,
    limit: u32,
    batchcount: u32,
    shared: u32,
    objects_per_slab: u32,
    cache_order: u32,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct kmem_object_info {
    kp_ptr: *mut c_void,
    kp_slab: *mut slab,
    kp_objp: *mut c_void,
    kp_data_offset: u64,
    kp_slab_cache: *mut kmem_cache,
    kp_ret: *mut c_void,
    kp_stack: [*mut c_void; KS_ADDRS_COUNT],
    kp_free_stack: [*mut c_void; KS_ADDRS_COUNT],
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
const SLAB_PANIC: u32 = 1 << (_slab_flag_bits::_SLAB_PANIC as u32);
const SLAB_TYPESAFE_BY_RCU: u32 = 1 << (_slab_flag_bits::_SLAB_TYPESAFE_BY_RCU as u32);
const SLAB_NOLEAKTRACE: u32 = 1 << (_slab_flag_bits::_SLAB_NOLEAKTRACE as u32);
const SLAB_CONSISTENCY_CHECKS: u32 = 1 << (_slab_flag_bits::_SLAB_CONSISTENCY_CHECKS as u32);
const SLAB_NO_MERGE: u32 = 1 << (_slab_flag_bits::_SLAB_NO_MERGE as u32);
const SLAB_RECLAIM_ACCOUNT: u32 = 1 << (_slab_flag_bits::_SLAB_RECLAIM_ACCOUNT as u32);
const SLAB_CACHE_DMA: u32 = 1 << (_slab_flag_bits::_SLAB_CACHE_DMA as u32);
const SLAB_CACHE_DMA32: u32 = 1 << (_slab_flag_bits::_SLAB_CACHE_DMA32 as u32);
const SLAB_ACCOUNT: u32 = 1 << (_slab_flag_bits::_SLAB_ACCOUNT as u32);
const SLAB_KMALLOC: u32 = 1 << (_slab_flag_bits::_SLAB_KMALLOC as u32);



const SLAB_NEVER_MERGE: u32 = SLAB_RED_ZONE | SLAB_POISON | SLAB_STORE_USER |
                                SLAB_TRACE | SLAB_TYPESAFE_BY_RCU | SLAB_NOLEAKTRACE |
                                 SLAB_NO_MERGE;

const SLAB_MERGE_SAME: u32 =  SLAB_RECLAIM_ACCOUNT | SLAB_CACHE_DMA | 
                                SLAB_CACHE_DMA32 | SLAB_ACCOUNT;

const SLAB_CORE_FLAGS: u32 = SLAB_HWCACHE_ALIGN | SLAB_CACHE_DMA | 
                                    SLAB_CACHE_DMA32 | SLAB_PANIC | 
                                    SLAB_TYPESAFE_BY_RCU;

const SLAB_FLAGS_PERMITTED: u32 = SLAB_CORE_FLAGS | 
                                    SLAB_RED_ZONE | 
                                    SLAB_POISON | 
                                    SLAB_STORE_USER | 
                                    SLAB_TRACE | 
                                    SLAB_CONSISTENCY_CHECKS | 
                                    SLAB_NOLEAKTRACE | 
                                    SLAB_RECLAIM_ACCOUNT | 
                                    SLAB_ACCOUNT | 
                                    SLAB_KMALLOC | 
                                    SLAB_NO_MERGE | 
                                    SLAB_NO_USER_FLAGS;
const SLAB_DEBUG_FLAGS: u32 = SLAB_RED_ZONE | SLAB_POISON | SLAB_STORE_USER |
                                        SLAB_TRACE | SLAB_CONSISTENCY_CHECKS;

const SLAB_CACHE_FLAGS: u32 = SLAB_NOLEAKTRACE | SLAB_RECLAIM_ACCOUNT | 
                                            SLAB_ACCOUNT |  SLAB_NO_USER_FLAGS | 
                                            SLAB_KMALLOC | SLAB_NO_MERGE;

const CACHE_CREATE_MASK: u32 =  SLAB_CORE_FLAGS | SLAB_DEBUG_FLAGS | SLAB_CACHE_FLAGS;


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

fn err_ptr(err: isize) -> *mut () {
    err as *mut ()
}

fn ptr_err(ptr: *const ()) -> isize {
    ptr as isize
}

fn is_err(ptr: *const ()) -> bool {
    (ptr as usize) > usize::MAX - 1000
}

fn kmem_cache_sanity_check(name: *const c_char, size: u32) -> i32
{
	return 0;
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

 extern "C" {
    pub static slab_caches: bindings::LIST_HEAD;
    pub static kmem_cache_obj: *mut kmem_cache;
    pub static slab_mutex: bindings::mutex;
    pub static slub_debug_enabled: bindings::static_key_false;

    pub fn kmem_cache_alloc_noprof(cachep: *mut kmem_cache, flags: gfp_t) -> *mut core::ffi::c_void;
    pub fn __kmem_cache_create(cache: *mut kmem_cache, flags: slab_flags_t) -> u32;
    pub fn kmem_cache_free(s: *mut kmem_cache, objp: *const c_void);
    pub fn dump_stack();
    pub fn stack_depot_init();
    pub fn mutex_unlock(m: *mut bindings::slab_mutex);
    pub fn mutex_lock(m: *mut bindings::slab_mutex);
    pub fn static_key_enable(key: *mut bindings::static_key);
    pub fn kstrdup_const(s: *const c_char, gfp: gfp_t) -> *const c_char;
    pub fn __kmem_cache_alias(name: *const c_char, size: u32, align: u32,
                              flags: slab_flags_t, ctor: *const c_void) -> *mut kmem_cache;
    pub fn  kfree_const(x: *const c_void);
}

#[no_mangle]
pub extern "C" fn find_mergeable(size: u32, mut align: u32, mut flags: slab_flags_t,
                                 name: *const c_char, ctor: *const c_void) -> *mut kmem_cache
{

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

    unsafe {
        let s: *mut kmem_cache = slab_caches.prev;
        while s != slab_caches {
            if slab_unmergeable(s) != 0 {
                s = s.prev;
                continue;
            }

            if size > s.size {
                s = s.prev;
                continue;
            }

            if (flags & SLAB_MERGE_SAME) != (s.flags & SLAB_MERGE_SAME) {
                s = s.prev;
                continue;
            }
            /*
            * Check if alignment is compatible.
            * Courtesy of Adrian Drzewiecki
            */
            if (s.size & !(align - 1)) != s.size {
                s = s.prev;
                continue;
            }

            if s.size - size >= core::mem::size_of::<*const ()>() as u32 {
                s = s.prev;
                continue;
            }

            s
        }
    }

    return core::ptr::null_mut();
}


fn create_cache(name: *const c_char,
    object_size: u32, align: u32,
    flags: slab_flags_t, useroffset: u32,
    usersize: u32, ctor: *mut c_void,
    root_cache: *const kmem_cache) -> *mut kmem_cache
{
    

    /*
    // This would be annoying to port
    if (WARN_ON(useroffset + usersize > object_size))
        useroffset = usersize = 0;
    */
    let mut err: u32 = !ENOMEM;
    // 
    // #define kmem_cache_alloc(...)			alloc_hooks(kmem_cache_alloc_noprof(__VA_ARGS__))
    // kmem_cache_alloc(_k, (_flags)|__GFP_ZERO)
    // The hook is need, but annoyting see <linux/alloc_tag.h>
    // TODO: what to do with alloc_hooks
    let mut s: *mut kmem_cache = core::ptr::null_mut();
    unsafe {
        s = kmem_cache_alloc_noprof(kmem_cache_obj, core::mem::transmute(Gfp::GFP_KERNEL as u32 | 0x80)) as *mut kmem_cache;
    }
    if s != core::ptr::null_mut() {
        return err as *mut kmem_cache
    }
    (*s).name = name;
    (*s).object_size = object_size;
    (*s).size = object_size;
    (*s).align = align;
    (*s).ctor = ctor;
    if CONFIG_HARDENED_USERCOPY {
        (*s).useroffset = useroffset;
        (*s).usersize = usersize;
    }

    err = __kmem_cache_create(s, flags);
    if err != 0 {
        unsafe {
            kmem_cache_free(kmem_cache_obj, s as *const c_void);
        }
        return err as *mut kmem_cache
    }

    (*s).refcount = 1;
    slab_caches.next.prev = (*s).list;
    (*s).list.next = slab_caches.next;
    (*s).list.prev = slab_caches;
    slab_caches.next = (*s).list;
    s
}

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
 
fn kmem_cache_create_usercopy(name: *const c_char,
           size: u32, align: u32, flags: slab_flags_t,
           useroffset: u32, usersize: u32,
           ctor: *mut c_void) -> *mut kmem_cache
 {
    let mut s: *mut kmem_cache = core::ptr::null_mut();
 
    if CONFIG_SLUB_DEBUG {
        /*
        * If no slab_debug was enabled globally, the static key is not yet
        * enabled by setup_slub_debug(). Enable it if the cache is being
        * created with any of the debugging flags passed explicitly.
        * It's also possible that this is the first cache created with
        * SLAB_STORE_USER and we should init stack_depot for it.
        */
        if flags & SLAB_DEBUG_FLAGS != 0 {
            unsafe {
                static_key_enable(&slub_debug_enabled);
            }
        }
        if flags & SLAB_STORE_USER != 0 {
            unsafe {
                stack_depot_init();
            }
        }
    }
 
    unsafe {
        mutex_lock(&slab_mutex);
    }
     let mut err: i32 = 0;
     err = kmem_cache_sanity_check(name, size);
     if err != 0 {
        unsafe {
            mutex_unlock(&slab_mutex);

            if flags & SLAB_PANIC != 0 {
                panic!(
                    "kmem_cache_create_usercopy: Failed to create slab '{}'. Error {}\n",
                    CStr::from_ptr(name).to_str().unwrap_or("<invalid UTF-8>"), err
                );
            }
            else {
                dump_stack();
            }
            return core::ptr::null_mut();
        }
     }
 
     /* Refuse requests with allocator specific flags */
     if flags & !SLAB_FLAGS_PERMITTED != 0 {
         err = -(EINVAL as i32);
         unsafe {
            mutex_unlock(&slab_mutex);

            if flags & SLAB_PANIC != 0 {
                panic!(
                    "kmem_cache_create_usercopy: Failed to create slab '{}'. Error {}\n",
                    CStr::from_ptr(name).to_str().unwrap_or("<invalid UTF-8>"), err
                );
            }
            else {
                dump_stack();
            }
            return core::ptr::null_mut();
        }
    }
 
     /*
      * Some allocators will constraint the set of valid flags to a subset
      * of all flags. We expect them to define CACHE_CREATE_MASK in this
      * case, and we'll just provide them with a sanitized version of the
      * passed flags.
      */
     flags &= CACHE_CREATE_MASK;
 
     /* Fail closed on bad usersize of useroffset values. */
    if (!CONFIG_HARDENED_USERCOPY) ||
         (usersize == 0 && useroffset != 0) ||
         (size < usersize || size - usersize < useroffset) {
        usersize = 0;
        useroffset = 0;
    }
 
     if usersize == 0 {
        unsafe {
            s = __kmem_cache_alias(name, size, align, flags, ctor);
        }
     }
     if s != core::ptr::null_mut() {
        unsafe {
            mutex_unlock(&slab_mutex);

            if flags & SLAB_PANIC != 0 {
                panic!(
                    "kmem_cache_create_usercopy: Failed to create slab '{}'. Error {}\n",
                    CStr::from_ptr(name).to_str().unwrap_or("<invalid UTF-8>"), err
                );
            }
            else {
                dump_stack();
            }
            return core::ptr::null_mut();
        }
    }
 
     let cache_name: *const c_char = kstrdup_const(name, gfp_t::GFP_KERNEL);
     if cache_name != core::ptr::null_mut() {
         err = -(ENOMEM as i32);
         unsafe {
            mutex_unlock(&slab_mutex);

            if flags & SLAB_PANIC != 0 {
                panic!(
                    "kmem_cache_create_usercopy: Failed to create slab '{}'. Error {}\n",
                    CStr::from_ptr(name).to_str().unwrap_or("<invalid UTF-8>"), err
                );
            }
            else {
                dump_stack();
            }
            return core::ptr::null_mut();
        }
     }
 
    s = create_cache(cache_name, size,
              calculate_alignment(flags, align, size),
              flags, useroffset, usersize, ctor, core::ptr::null_mut());
     if is_err(s as *const ()) {
         err = ptr_err(s as *const ()) as i32;
         unsafe {
            kfree_const(cache_name as *const c_void);
         }
     }
     if err != 0 {
        unsafe {
            mutex_unlock(&slab_mutex);

            if flags & SLAB_PANIC != 0 {
                panic!(
                    "kmem_cache_create_usercopy: Failed to create slab '{}'. Error {}\n",
                    CStr::from_ptr(name).to_str().unwrap_or("<invalid UTF-8>"), err
                );
            }
            else {
                dump_stack();
            }
            return core::ptr::null_mut();
        }
     }
     
     unsafe {
         mutex_unlock(&slab_mutex);
     }
     s
 }

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