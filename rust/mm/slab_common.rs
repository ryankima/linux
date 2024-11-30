#![no_std]
use core::ffi::{c_void, c_char, CStr};
use core::marker::Copy;
use core::convert::From;
use core::ptr::{addr_of, addr_of_mut};

extern crate bindings;

/**************************** BEGIN DEFINES DEFINITIONS ********************************/
const CONFIG_SLAB_BUCKETS: bool = true;
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
const CONFIG_RANDOM_KMALLOC_CACHES: bool = true;

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

const LIST_POISON1: *mut kmem_cache = 0x100 as *mut kmem_cache;
const LIST_POISON2: *mut kmem_cache = 0x122 as *mut kmem_cache;

const RANDOM_KMALLOC_CACHES_NR: u32 =	0;

const ARCH_KMALLOC_MINALIGN: usize = core::mem::align_of::<u64>();

/**************************** BEGIN TYPE DEFINITIONS ********************************/
#[allow(non_camel_case_types)]
type slab_flags_t = u32;

#[allow(non_camel_case_types)]
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



const KMALLOC_NORMAL: u32 = 0;
const KMALLOC_RANDOM_START: u32 = 0; // Same as KMALLOC_NORMAL
const KMALLOC_RANDOM_END: u32 = KMALLOC_RANDOM_START + RANDOM_KMALLOC_CACHES_NR;
const KMALLOC_RECLAIM: u32 = KMALLOC_RANDOM_END + 1;
const KMALLOC_DMA: u32 = KMALLOC_RANDOM_END + 2;
const KMALLOC_CGROUP: u32 = KMALLOC_RANDOM_END + 3;
const NR_KMALLOC_TYPES: u32 = KMALLOC_RANDOM_END + 4;

#[repr(C)]
pub struct kmalloc_info_struct {
    pub name: [*const c_char; NR_KMALLOC_TYPES as usize], // Array of const char pointers
    pub size: u32, // Matches `unsigned int` in C
}

// This comes from <linux/slab.h>
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
pub enum slab_state_t {
	DOWN,			/* No slab functionality yet */
	PARTIAL,		/* SLUB: kmem_cache_node available */
	UP,			/* Slab caches usable but not all extras yet */
	FULL			/* Everything is working */
}

const slab_nomerge: bool = true; // TODO: check if this is actually false
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

const PAGE_SHIFT: u32 = 12;
const MAX_PAGE_ORDER: u32 = 10;
const PAGE_SIZE: u32 = 1 << PAGE_SHIFT;
const KMALLOC_SHIFT_HIGH: u32 = PAGE_SHIFT + 1;
const KMALLOC_MAX_CACHE_SIZE: u32 =	1 << KMALLOC_SHIFT_HIGH;

const KMALLOC_SHIFT_MAX: u32 = MAX_PAGE_ORDER + PAGE_SHIFT;
const KMALLOC_MAX_SIZE: u32 = 1 << KMALLOC_SHIFT_MAX;
const KMALLOC_SHIFT_LOW: u32 =	3;
const KMALLOC_MIN_SIZE: u32 = 1 << KMALLOC_SHIFT_LOW;


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

fn kmem_cache_sanity_check(_name: *const c_char, _size: u32) -> i32
{
	return 0;
}

fn ffs(x: u32) -> u32 {
    if x == 0 {
        0
    } else {
        (x & !(x - 1)).trailing_zeros() + 1
    }
}

macro_rules! offsetof {
    ($type:ty, $field:ident) => {{
        let tmp: $type = core::mem::zeroed();
        let offset = &(*tmp).$field as *const _ as usize - &tmp as *const _ as usize;
        core::mem::forget(tmp); // Prevent deallocation of the zeroed memory
        offset
    }};
}

fn size_index_elem(bytes: u32) -> u32
{
	(bytes - 1) / 8
}

fn list_empty(head: *mut bindings::list_head) -> i32
{
    if unsafe {(*head).next == head} {
        return 1;
    }
    return 0;
}

fn INIT_LIST_HEAD(list: *mut bindings::list_head)
{
    unsafe {
        core::ptr::write_volatile((*list).next, *list);
        core::ptr::write_volatile((*list).next, *list);
    }
}

fn __list_splice(list: *const bindings::list_head,
    prev: *mut bindings::list_head,
    next: *mut bindings::list_head)
{
    unsafe {
        let first: *mut bindings::list_head = (*list).next;
        let last: *mut bindings::list_head = (*list).prev;

        (*first).prev = prev;
        (*prev).next = first;

        (*last).next = next;
        (*next).prev = last;
    }
}

fn list_splice_init(list: *mut bindings::list_head,
    head: *mut bindings::list_head)
{
    if list_empty(list) == 0 {
        unsafe {
            __list_splice(list, head, (*head).next);
        }
        INIT_LIST_HEAD(list);
    }
}



/*
 * Delete a list entry by making the prev/next entries
 * point to each other.
 *
 * This is only for internal list manipulation where we know
 * the prev/next entries already!
 */
fn __list_del(prev: *mut bindings::list_head, next: *mut bindings::list_head)
 {
    unsafe {
        (*next).prev = prev;
        (*prev).next = next;
    }
 }
 
 // This shouldn't be called directly, entry presumed to be not none
fn __list_del_entry(entry: *mut bindings::list_head)
 {
    unsafe {
     __list_del((*entry).prev, (*entry).next);
    }
 }
 
 /**
  * list_del - deletes entry from list.
  * @entry: the element to delete from the list.
  * Note: list_empty() on entry does not return true after this, the entry is
  * in an undefined state.
  */
fn list_del(entry: *mut bindings::list_head)
 {
     __list_del_entry(entry);
     unsafe {
        (*entry).next = LIST_POISON1 as *mut bindings::list_head;
        (*entry).prev = LIST_POISON2 as *mut bindings::list_head;
     }
 }

 /*
 * Insert a new entry between two known consecutive entries.
 *
 * This is only for internal list manipulation where we know
 * the prev/next entries already!
 */
fn __list_add(new: *mut bindings::list_head,
    prev: *mut bindings::list_head,
    next: *mut bindings::list_head)
{
    unsafe {
        (*next).prev = new;
        (*new).next = next;
        (*new).prev = prev;
        // TODO: double check this
        core::ptr::write_volatile(&mut (*prev).next, new);
    }
}

/**
* list_add - add a new entry
* @new: new entry to be added
* @head: list head to add it after
*
* Insert a new entry after the specified head.
* This is good for implementing stacks.
*/
fn list_add(new: *mut bindings::list_head, head: *mut bindings::list_head)
{
    unsafe {
    __list_add(new, head, (*head).next);
    }
}


/**
* list_add_tail - add a new entry
* @new: new entry to be added
* @head: list head to add it before
*
* Insert a new entry before the specified head.
* This is useful for implementing queues.
*/
fn list_add_tail(new: *mut bindings::list_head, head: *mut bindings::list_head)
{
    unsafe {
    __list_add(new, (*head).prev, head);
    }  
}
 

/**************************** BEGIN FUNCTION DEFINITIONS ********************************/
extern "C" {
    fn kmem_cache_flags(flags: slab_flags_t, name: *const c_char) -> slab_flags_t;
}


fn __kmalloc_index(size: usize, size_is_constant: bool) -> u32
{
    if size == 0 {
        return 0;
    }

    if size <= KMALLOC_MIN_SIZE.try_into().unwrap() {
        return KMALLOC_SHIFT_LOW;
    }

    if KMALLOC_MIN_SIZE <= 32 && size > 64 && size <= 96 {
        return 1;
    }
    if KMALLOC_MIN_SIZE <= 64 && size > 128 && size <= 192 {
        return 2;
    }
    if size <=          8 {return 3;}
    if size <=         16 {return 4;}
    if size <=         32 {return 5;}
    if size <=         64 {return 6;}
    if size <=        128 {return 7;}
    if size <=        256 {return 8;}
    if size <=        512 {return 9;}
    if size <=       1024 {return 10;}
    if size <=   2 * 1024 {return 11;}
    if size <=   4 * 1024 {return 12;}
    if size <=   8 * 1024 {return 13;}
    if size <=  16 * 1024 {return 14;}
    if size <=  32 * 1024 {return 15;}
    if size <=  64 * 1024 {return 16;}
    if size <= 128 * 1024 {return 17;}
    if size <= 256 * 1024 {return 18;}
    if size <= 512 * 1024 {return 19;}
    if size <= 1024 * 1024 {return 20;}
    if size <=  2 * 1024 * 1024 {return 21;}
    return u32::MAX;
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
    pub static mut slab_caches: bindings::list_head;
    pub static mut slab_caches_to_rcu_destroy: bindings::list_head;
    pub static mut slab_caches_to_rcu_destroy_work: bindings::work_struct;
    pub static mut kmem_cache: *mut kmem_cache;
    pub static mut slab_mutex: bindings::mutex;
    pub static mut slub_debug_enabled: bindings::static_key_false;
    pub static mut kmalloc_caches: [bindings::kmem_buckets; 4];
    pub static mut slab_state: slab_state_t;
    pub static mut kmem_buckets_cache: *mut kmem_cache;
    pub static mut kmalloc_size_index: [u8; 24];
    pub static kmalloc_info: [kmalloc_info_struct; 22];

    pub fn panic(fmt: *const c_char, ...);
    pub fn kmem_cache_alloc_noprof(cachep: *const kmem_cache, flags: gfp_t) -> *mut core::ffi::c_void;
    pub fn __kmem_cache_create(cache: *mut kmem_cache, flags: slab_flags_t) -> u32;
    pub fn kmem_cache_free(s: *mut kmem_cache, objp: *const c_void);
    pub fn dump_stack();
    pub fn stack_depot_init();
    pub fn mutex_unlock(m: *const bindings::mutex);
    pub fn mutex_lock(m: *const bindings::mutex);
    pub fn static_key_enable(key: *mut bindings::static_key_false);
    pub fn kstrdup_const(s: *const c_char, gfp: gfp_t) -> *const c_char;
    pub fn __kmem_cache_alias(name: *const c_char, size: u32, align: u32,
                              flags: slab_flags_t, ctor: *const c_void) -> *mut kmem_cache;
    pub fn  kfree_const(x: *const c_void);
    pub fn sysfs_slab_unlink(s: *mut kmem_cache);
    pub fn sysfs_slab_release(s: *mut kmem_cache);
    pub fn kfree(objp: *const c_void);
    pub fn strchr(s: *const c_char, c: i32) -> *const c_char;
    pub fn kasprintf(gfp_mask: u32, fmt: *const i8, ...) -> *mut i8;
    pub fn schedule_work_link(work: *mut bindings::work_struct) -> bool;
    pub fn pfn_valid(pfn: u32) -> i32;
    pub fn rcu_barrier();
    pub fn __kmem_cache_release(s: *mut kmem_cache);
    pub fn debugfs_slab_release(s: *mut kmem_cache);
    pub fn __kmem_cache_shutdown(s: *mut kmem_cache) -> i32;
    pub fn kfence_shutdown_cache(s: *mut kmem_cache);
    pub fn kasan_cache_shutdown_link(cache: *mut kmem_cache);
    pub fn kasan_cache_shrink_link(cache: *mut kmem_cache);
    pub fn __kmem_cache_shrink(s: *mut kmem_cache) -> i32;
    pub fn __kfence_obj_info(kpp: *mut kmem_object_info, object: *mut c_void, slab: *mut slab) -> bool;
    pub fn __kmem_obj_info(kpp: *mut kmem_object_info, object: *mut c_void, slab: *mut slab);
    pub fn virt_to_slab(addr: *const c_void) -> *mut slab;
    pub fn cpus_read_lock();
    pub fn cpus_read_unlock();
    pub fn get_order_link(s: u32) -> i32;
    pub fn kmalloc_slab_link(size: usize, b: *mut bindings::kmem_buckets, flags: gfp_t, caller: u32) -> *mut kmem_cache;
    pub fn kmem_cache_create_kernel_link() -> *mut kmem_cache;
    pub fn kmem_cache_create_nowait_link() -> *mut kmem_cache;

    pub fn dma_get_cache_alignment_link() -> u32;
    pub fn config_dma_bounce_unaligned_link() -> u32;
    pub fn arch_slab_minalign_link() -> u32;
    pub fn mem_cgroup_kmem_disabled_link() -> bool;

    // TODO: Remove these later
}

#[no_mangle]
pub extern "C" fn find_mergeable(size: u32, mut align: u32, mut flags: slab_flags_t,
                                 name: *const c_char, ctor: *const c_void) -> *mut kmem_cache
{
    if slab_nomerge {
        return core::ptr::null_mut();
    }

    // TODO: below might be cooked

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

    let mut s: *mut kmem_cache = unsafe { slab_caches.prev as *mut kmem_cache };
    while s != addr_of_mut!(slab_caches) as *mut kmem_cache {
        if slab_unmergeable(s) != 0 {
            unsafe { s = (*s).list.prev as *mut kmem_cache;}
            continue;
        }

        if size > unsafe {(*s).size} {
            unsafe { s = (*s).list.prev as *mut kmem_cache;}
            continue;
        }

        if (flags & SLAB_MERGE_SAME) != unsafe {(*s).flags & SLAB_MERGE_SAME} {
            unsafe { s = (*s).list.prev as *mut kmem_cache;}
            continue;
        }
        /*
        * Check if alignment is compatible.
        * Courtesy of Adrian Drzewiecki
        */
        if unsafe { (*s).size & !(align - 1) != (*s).size } {
            unsafe { s = (*s).list.prev as *mut kmem_cache;}
            continue;
        }

        if unsafe { (*s).size } - size >= core::mem::size_of::<*const ()>() as u32 {
            unsafe { s = (*s).list.prev as *mut kmem_cache;}
            continue;
        }

        return s;
    }

    return core::ptr::null_mut();
}

#[no_mangle]
pub extern "C" fn create_cache(name: *const c_char,
    object_size: u32, align: u32,
    flags: slab_flags_t, useroffset: u32,
    usersize: u32, ctor: *mut c_void,
    _root_cache: *const kmem_cache) -> *mut kmem_cache
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
    let s: *mut kmem_cache;
    unsafe {
        s = kmem_cache_create_kernel_link();
    }
    if s == core::ptr::null_mut() {
        return err as *mut kmem_cache
    }
    unsafe {
        (*s).name = name;
        (*s).object_size = object_size;
        (*s).size = object_size;
        (*s).align = align;
        (*s).ctor = ctor;
    }
    if CONFIG_HARDENED_USERCOPY {
        unsafe {
            (*s).useroffset = useroffset;
            (*s).usersize = usersize;
        }
    }

    err = unsafe {__kmem_cache_create(s, flags)};
    if err != 0 {
        unsafe {
            kmem_cache_free(kmem_cache, s as *const c_void);
        }
        return err as *mut kmem_cache
    }

    unsafe {
        (*s).refcount = 1;
        (*slab_caches.next).prev = &mut (*s).list;
        (*s).list.next = slab_caches.next;
        (*s).list.prev = addr_of_mut!(slab_caches);
        slab_caches.next = &mut (*s).list;
    }
    return s;
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
 #[no_mangle]
pub extern "C" fn kmem_cache_create_usercopy(name: *const c_char,
           size: u32, align: u32, mut flags: slab_flags_t,
           mut useroffset: u32, mut usersize: u32,
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
                static_key_enable(addr_of_mut!(slub_debug_enabled));
            }
        }
        if flags & SLAB_STORE_USER != 0 {
            unsafe {
                stack_depot_init();
            }
        }
    }
 
    unsafe {
        mutex_lock(addr_of!(slab_mutex));
    }
     let mut err: i32 = kmem_cache_sanity_check(name, size);
     if err != 0 {
        unsafe {
            mutex_unlock(addr_of!(slab_mutex));

            if flags & SLAB_PANIC != 0 {
                panic("kmem_cache_create_usercopy: Failed to create slab".as_ptr() as *const i8);
                // panic!(
                //     "kmem_cache_create_usercopy: Failed to create slab '{}'. Error {}\n",
                //     CStr::from_ptr(name).to_str().unwrap_or("<invalid UTF-8>"), err
                // );
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
            mutex_unlock(addr_of!(slab_mutex));

            if flags & SLAB_PANIC != 0 {
                panic("kmem_cache_create_usercopy: Failed to create slab".as_ptr() as *const i8);
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
            mutex_unlock(addr_of!(slab_mutex));

            if flags & SLAB_PANIC != 0 {
                panic("kmem_cache_create_usercopy: Failed to create slab".as_ptr() as *const i8);
            }
            else {
                dump_stack();
            }
            return core::ptr::null_mut();
        }
    }
 
     let cache_name: *const c_char = unsafe {kstrdup_const(name, gfp_t::GFP_KERNEL)};
     if cache_name == core::ptr::null_mut() {
         err = -(ENOMEM as i32);
         unsafe {
            mutex_unlock(addr_of!(slab_mutex));

            if flags & SLAB_PANIC != 0 {
                panic("kmem_cache_create_usercopy: Failed to create slab".as_ptr() as *const i8);
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
            mutex_unlock(addr_of!(slab_mutex));

            if flags & SLAB_PANIC != 0 {
                panic("kmem_cache_create_usercopy: Failed to create slab".as_ptr() as *const i8);
            }
            else {
                dump_stack();
            }
            return core::ptr::null_mut();
        }
     }
     
     unsafe {
         mutex_unlock(addr_of!(slab_mutex));
     }
     s
 }

 

 /**
 * kmem_cache_create - Create a cache.
 * @name: A string which is used in /proc/slabinfo to identify this cache.
 * @size: The size of objects to be created in this cache.
 * @align: The required alignment for the objects.
 * @flags: SLAB flags
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

 #[no_mangle]
 pub extern "C" fn kmem_cache_create(name: *const c_char,
    size: u32, align: u32, flags: slab_flags_t,
    ctor: *mut c_void) -> *mut kmem_cache
{
	kmem_cache_create_usercopy(name, size, align, flags, 0, 0, ctor)
}

/**
 * kmem_buckets_create - Create a set of caches that handle dynamic sized
 *			 allocations via kmem_buckets_alloc()
 * @name: A prefix string which is used in /proc/slabinfo to identify this
 *	  cache. The individual caches with have their sizes as the suffix.
 * @flags: SLAB flags (see kmem_cache_create() for details).
 * @useroffset: Starting offset within an allocation that may be copied
 *		to/from userspace.
 * @usersize: How many bytes, starting at @useroffset, may be copied
 *		to/from userspace.
 * @ctor: A constructor for the objects, run when new allocations are made.
 *
 * Cannot be called within an interrupt, but can be interrupted.
 *
 * Return: a pointer to the cache on success, NULL on failure. When
 * CONFIG_SLAB_BUCKETS is not enabled, ZERO_SIZE_PTR is returned, and
 * subsequent calls to kmem_buckets_alloc() will fall back to kmalloc().
 * (i.e. callers only need to check for NULL on failure.)
 */
 #[no_mangle]
pub extern "C" fn kmem_buckets_create(name: *const c_char, mut flags: slab_flags_t,
    useroffset: u32, usersize: u32, ctor: *mut c_void) -> *mut bindings::kmem_buckets
{
    let b: *mut bindings::kmem_buckets;
    let mut idx: i32 = 0;

	/*
	 * When the separate buckets API is not built in, just return
	 * a non-NULL value for the kmem_buckets pointer, which will be
	 * unused when performing allocations.
	 */
    if !CONFIG_SLAB_BUCKETS {
        return 16 as *mut bindings::kmem_buckets;
    }

    if addr_of_mut!(kmem_buckets_cache) == core::ptr::null_mut() {
        return core::ptr::null_mut();
    }

    // #define kmem_cache_alloc(...)			alloc_hooks(kmem_cache_alloc_noprof(__VA_ARGS__))
    // kmem_cache_alloc(_k, (_flags)|__GFP_ZERO)
    // The hook is need, but annoyting see <linux/alloc_tag.h>
    // TODO: what to do with alloc_hooks
    let mut s: *mut kmem_cache;
    unsafe {
        b = kmem_cache_alloc_noprof(kmem_buckets_cache, core::mem::transmute(Gfp::GFP_KERNEL as u32 | 0x80)) as *mut bindings::kmem_buckets;
    }

    if b == core::ptr::null_mut() {
        return core::ptr::null_mut();
    }

    flags |= SLAB_NO_MERGE;
    

     while idx < (KMALLOC_SHIFT_HIGH + 1) as i32 {
        let mut short_size: *const c_char;
        let cache_name: *const c_char;
        let cache_useroffset: u32; 
        let cache_usersize: u32;
        let size: u32;
        
        if unsafe {kmalloc_caches[0][idx as usize].is_null()} {
            idx += 1;
            continue;
        }
        size = unsafe{(*(kmalloc_caches[0][idx as usize] as *mut kmem_cache)).object_size};
        if size == 0 {
            idx += 1;
            continue;
        }
        

        unsafe {
            short_size = strchr((*(kmalloc_caches[0][idx as usize] as *mut kmem_cache)).name, '-' as i32);
        }
        if short_size == core::ptr::null_mut() {
            idx = 0;
            while idx < (KMALLOC_SHIFT_HIGH + 1) as i32 {
                unsafe {kmem_cache_destroy((*b)[idx as usize] as *mut kmem_cache)};
                idx += 1;
            }
            unsafe {
                kfree(b as *const c_void);
            }

            return core::ptr::null_mut();
        }

        unsafe {
            cache_name = kasprintf(gfp_t::GFP_KERNEL as u32, "%s-%s".as_ptr() as *const i8, name, short_size.add(1));
        }
        if cache_name == core::ptr::null_mut() {
            idx = 0;
            while idx < (KMALLOC_SHIFT_HIGH + 1) as i32 {
                unsafe {kmem_cache_destroy((*b)[idx as usize] as *mut kmem_cache)};
                idx += 1;
            }
            unsafe {
                kfree(b as *const c_void);
            }

            return core::ptr::null_mut();
        }

        if useroffset >= size {
            cache_useroffset = 0;
            cache_usersize = 0;
        } else {
            cache_useroffset = useroffset;
            cache_usersize = core::cmp::min(size - cache_useroffset, usersize);
        }
        // This is a terrible cast
        unsafe {
            (*b)[idx as usize] = kmem_cache_create_usercopy(cache_name, size,
                    0, flags, cache_useroffset,
                    cache_usersize, ctor) as *mut bindings::kmem_cache;
                
            kfree(cache_name as *const c_void);
        }
        if unsafe {(*b)[idx as usize]} == core::ptr::null_mut() {
            idx = 0;
            while idx < (KMALLOC_SHIFT_HIGH + 1) as i32{
                unsafe {kmem_cache_destroy((*b)[idx as usize] as *mut kmem_cache)};
                idx += 1;
            }
            unsafe {
                kfree(b as *const c_void);
            }

            return core::ptr::null_mut();
        }
        idx += 1;
    }
 b
}


/*
 * For a given kmem_cache, kmem_cache_destroy() should only be called
 * once or there will be a use-after-free problem. The actual deletion
 * and release of the kobject does not need slab_mutex or cpu_hotplug_lock
 * protection. So they are now done without holding those locks.
 *
 * Note that there will be a slight delay in the deletion of sysfs files
 * if kmem_cache_release() is called indrectly from a work function.
 */
 #[no_mangle]
pub extern "C" fn kmem_cache_release(s: *mut kmem_cache)
 {
    // slab_state is static, so it's unsafe
    unsafe {
        if slab_state >= slab_state_t::FULL {
            sysfs_slab_unlink(s);
            sysfs_slab_release(s);
        } else {
            slab_kmem_cache_release(s);
        }
    }
 }

#[no_mangle]
pub extern "C" fn slab_caches_to_rcu_destroy_workfn(work: *mut bindings::work_struct)
{
    let mut to_destroy: bindings::list_head = bindings::list_head {
        next: core::ptr::null_mut(), 
        prev: core::ptr::null_mut()
    };

    to_destroy.next = &mut to_destroy;
    to_destroy.prev = &mut to_destroy;
    
	/*
	 * On destruction, SLAB_TYPESAFE_BY_RCU kmem_caches are put on the
	 * @slab_caches_to_rcu_destroy list.  The slab pages are freed
	 * through RCU and the associated kmem_cache are dereferenced
	 * while freeing the pages, so the kmem_caches should be freed only
	 * after the pending RCU operations are finished.  As rcu_barrier()
	 * is a pretty slow operation, we batch all pending destructions
	 * asynchronously.
	 */
    unsafe {
        mutex_lock(addr_of!(slab_mutex));
        list_splice_init(addr_of_mut!(slab_caches_to_rcu_destroy), &mut to_destroy);
        mutex_unlock(addr_of!(slab_mutex));

        if list_empty(&mut to_destroy) != 0 {
            return;
        }

        rcu_barrier();
    }

    let mut s: *mut kmem_cache = unsafe{to_destroy.next.sub(offsetof!(*mut kmem_cache, list)) as *mut kmem_cache};
    let mut s2: *mut kmem_cache = unsafe{(*s).list.next.sub(offsetof!(*mut kmem_cache, list)) as *mut kmem_cache};
    while unsafe {&(*s).list as *const bindings::list_head != &to_destroy as *const bindings::list_head} {
        unsafe {
            debugfs_slab_release(s);
            kfence_shutdown_cache(s);
        }
        kmem_cache_release(s);

        s = s2;
        unsafe {
            s2 = (*s).list.next.sub(offsetof!(*mut kmem_cache, list)) as *mut kmem_cache;
        }
    }
}

#[no_mangle]
pub extern "C" fn shutdown_cache(s: *mut kmem_cache) -> i32
{
	/* free asan quarantined objects */
    unsafe {
	    kasan_cache_shutdown_link(s);
    
        if __kmem_cache_shutdown(s) != 0 {
            return -(EBUSY as i32);
        }
    }

	list_del(&mut unsafe {(*s).list});

	if unsafe {(*s).flags} & SLAB_TYPESAFE_BY_RCU != 0 {
        unsafe {
		    list_add_tail(&mut (*s).list, addr_of_mut!(slab_caches_to_rcu_destroy));
		    schedule_work_link(addr_of_mut!(slab_caches_to_rcu_destroy_work));
        }
	} else {
        unsafe {
            kfence_shutdown_cache(s);
            debugfs_slab_release(s);
        }
	}

	return 0;
}

#[no_mangle]
pub extern "C" fn slab_kmem_cache_release(s: *mut kmem_cache)
{
    unsafe {
        __kmem_cache_release(s);
        kfree_const((*s).name as *const c_void);
        kmem_cache_free(kmem_cache, s as *const c_void);
    }
}

#[no_mangle]
pub extern "C" fn kmem_cache_destroy(s: *mut kmem_cache)
{
    if s == core::ptr::null_mut() {
        return;
    }

	let mut err: i32 = -(EBUSY as i32);
	let rcu_set: bool = unsafe {(*s).flags & SLAB_TYPESAFE_BY_RCU != 0};
    
    unsafe {
        cpus_read_lock();
        mutex_lock(addr_of!(slab_mutex));
    }
    
	unsafe{(*s).refcount -= 1;}
    
	if unsafe {(*s).refcount != 0} {
        unsafe {
            mutex_unlock(addr_of!(slab_mutex));
            cpus_read_unlock();
        }
        if err == 0 && !rcu_set {
            kmem_cache_release(s);
        }
        return
    }
    
	err = shutdown_cache(s);
    
    unsafe {
        mutex_unlock(addr_of!(slab_mutex));
        cpus_read_unlock();
    }
    if err == 0 && !rcu_set {
        kmem_cache_release(s);
    }
    
}

/**
 * kmem_cache_shrink - Shrink a cache.
 * @cachep: The cache to shrink.
 *
 * Releases as many slabs as possible for a cache.
 * To help debugging, a zero exit status indicates all slabs were released.
 *
 * Return: %0 if all slabs were released, non-zero otherwise
 */
 #[no_mangle]
 pub extern "C" fn kmem_cache_shrink(cachep: *mut kmem_cache) -> i32
 {
    unsafe {
        kasan_cache_shrink_link(cachep);
        __kmem_cache_shrink(cachep)
    }
 }
 
 #[no_mangle]
 pub extern "C" fn slab_is_available(_: c_void) -> bool
 {
    unsafe {
        return slab_state >= slab_state_t::UP;
    }
 }

#[no_mangle]
pub extern "C" fn kmem_obj_info(kpp: *mut kmem_object_info, object: *mut c_void, slab: *mut slab)
{
    unsafe {
        if __kfence_obj_info(kpp, object, slab) {
            return;
        }
        __kmem_obj_info(kpp, object, slab);
    }
}


/* Create a cache during boot when no slab services are available yet */
#[no_mangle]
pub extern "C" fn create_boot_cache(s: *mut kmem_cache, name: *const c_char,
		size: u32, flags: slab_flags_t,
		useroffset: u32, usersize: u32)
{
	let err: i32;
	let mut align: u32 = ARCH_KMALLOC_MINALIGN as u32;

	/*
	 * kmalloc caches guarantee alignment of at least the largest
	 * power-of-two divisor of the size. For power-of-two sizes,
	 * it is the size itself.
	 */
	if flags & SLAB_KMALLOC != 0 {
		align = core::cmp::max(align, 1 << (ffs(size) - 1));
    }

    unsafe {
        (*s).name = name;
        (*s).size = size;
        (*s).object_size = size;
        (*s).align = calculate_alignment(flags, align, size);

        if CONFIG_HARDENED_USERCOPY {
            (*s).useroffset = useroffset;
            (*s).usersize = usersize;
        }
    }

    unsafe {
	    err = __kmem_cache_create(s, flags) as i32;
    }

	if err != 0 {
        unsafe {
		    panic("Creation of kmalloc slab %s size=%d failed. Reason %d\n".as_ptr() as *const i8,
                name, size, err);
        }
    }

    unsafe {
	    (*s).refcount = -1;	/* Exempt from merging for now */
    }
}

#[no_mangle]
pub extern "C" fn create_kmalloc_cache(name: *const c_char,
						      size: u32, flags: slab_flags_t) -> *mut kmem_cache
{
    
    let s: *mut kmem_cache;
    
    unsafe {
        s = kmem_cache_create_nowait_link();
    }
    
	if s == core::ptr::null_mut() {
        unsafe {
		    panic("Out of memory when creating slab %s\n".as_ptr() as *const i8, name);
        }
    }

	create_boot_cache(s, name, size, flags | SLAB_KMALLOC, 0, size);
	unsafe {list_add(&mut (*s).list as *mut bindings::list_head, addr_of_mut!(slab_caches) as *mut bindings::list_head)};
	unsafe {(*s).refcount = 1};
    
	return s;
}

#[no_mangle]
extern "C" fn kmalloc_size_roundup(size: usize) -> usize {
    if size != 0 && size as u32 <= KMALLOC_MAX_CACHE_SIZE {
        /*
		 * The flags don't matter since size_index is common to all.
		 * Neither does the caller for just getting ->object_size.
		 */
        unsafe {
            let slab = kmalloc_slab_link(size, core::ptr::null_mut(), gfp_t::GFP_KERNEL, 0);
            if !slab.is_null() {
                return (*slab).object_size.try_into().unwrap();
            }
        }
    }

    if size != 0 && size as u32 <= KMALLOC_MAX_SIZE {
        return (PAGE_SIZE << unsafe { get_order_link(size as u32) }).try_into().unwrap();
    }

    // Return the size for 0 or very large values.
    size
}


/*
 * Patch up the size_index table if we have strange large alignment
 * requirements for the kmalloc array. This is only the case for
 * MIPS it seems. The standard arches will not generate any code here.
 *
 * Largest permitted alignment is 256 bytes due to the way we
 * handle the index determination for the smaller caches.
 *
 * Make sure that nothing crazy happens if someone starts tinkering
 * around with ARCH_KMALLOC_MINALIGN
 */

#[no_mangle]
pub extern "C" fn setup_kmalloc_cache_index_table(_: c_void)
 {
     let mut i: u32 = 8;
 
     while i < KMALLOC_MIN_SIZE {
         let elem: u32 = size_index_elem(i);
 
         if elem >= 24 {
             break;
         }
         unsafe {
            kmalloc_size_index[elem as usize] = KMALLOC_SHIFT_LOW as u8;
         }

         i += 8;
     }
 
     if KMALLOC_MIN_SIZE >= 64 {
         /*
          * The 96 byte sized cache is not used if the alignment
          * is 64 byte.
          */
         i = 64 + 8;
         while i <= 96 {
            unsafe {
                kmalloc_size_index[size_index_elem(i) as usize] = 7;
            }
            i += 8;
         }
 
     }
 
     if KMALLOC_MIN_SIZE >= 128 {
         /*
          * The 192 byte sized cache is not used if the alignment
          * is 128 byte. Redirect kmalloc to use the 256 byte cache
          * instead.
          */
          i = 128 + 8;
         while i <= 192 {
            unsafe {
                kmalloc_size_index[size_index_elem(i) as usize] = 8;
            }
             i += 8;
         }
     }
 }
 
 #[no_mangle]
 pub extern "C" fn __kmalloc_minalign() -> u32 {
    let mut minalign: u32 = unsafe {dma_get_cache_alignment_link()};

    if unsafe {config_dma_bounce_unaligned_link() != 0} {
        minalign = ARCH_KMALLOC_MINALIGN as u32;
    }

    core::cmp::max(minalign, unsafe {arch_slab_minalign_link()})
 }
 
 #[no_mangle]
 pub extern "C" fn new_kmalloc_cache(idx: i32, t: bindings::kmalloc_cache_type) {
    let mut flags: slab_flags_t = 0;
    let minalign: u32 = __kmalloc_minalign();
    let mut aligned_size: u32 = unsafe {kmalloc_info.get_unchecked(idx as usize).size};
    let mut aligned_idx: i32 = idx;

    if KMALLOC_RECLAIM != KMALLOC_NORMAL && t == KMALLOC_RECLAIM {
        flags |= SLAB_RECLAIM_ACCOUNT;
    } else if bindings::CONFIG_MEMCG != 0 && t == KMALLOC_CGROUP {
        if (unsafe {mem_cgroup_kmem_disabled_link()}) {
            unsafe {*kmalloc_caches.get_unchecked_mut(t as usize).get_unchecked_mut(aligned_idx as usize) 
                = *kmalloc_caches.get_unchecked_mut(KMALLOC_NORMAL as usize).get_unchecked_mut(idx as usize);}
            return;
        }
        flags |= SLAB_ACCOUNT;
    } else if bindings::CONFIG_ZONE_DMA != 0 && t == KMALLOC_DMA {
        flags |= SLAB_CACHE_DMA;
    }

    if CONFIG_RANDOM_KMALLOC_CACHES {
        if t >= KMALLOC_RANDOM_START && t <= KMALLOC_RANDOM_END {
            flags |= SLAB_NO_MERGE;
        }
    }

    /*
      * If CONFIG_MEMCG is enabled, disable cache merging for
      * KMALLOC_NORMAL caches.
      */
    if bindings::CONFIG_MEMCG != 0 && t == KMALLOC_NORMAL {
        flags |= SLAB_NO_MERGE;
    }

    if minalign > ARCH_KMALLOC_MINALIGN.try_into().unwrap() {
        aligned_size = align_macro(aligned_size, minalign);
        aligned_idx = __kmalloc_index(aligned_size as usize, false) as i32;
    }

    if unsafe {*kmalloc_caches.get_unchecked(t as usize).get_unchecked(aligned_idx as usize)} == core::ptr::null_mut() {
        unsafe {kmalloc_caches[t as usize][aligned_idx as usize] = create_kmalloc_cache(
                    kmalloc_info[aligned_idx as usize].name[t as usize],
                    aligned_size, flags) as *mut bindings::kmem_cache;}
    }
    if idx != aligned_idx  {
        unsafe {*kmalloc_caches.get_unchecked_mut(t as usize).get_unchecked_mut(aligned_idx as usize) = kmalloc_caches[t as usize][aligned_idx as usize];}
    }
 }
/*
 /*
  * Create the kmalloc array. Some of the regular kmalloc arrays
  * may already have been created because they were needed to
  * enable allocations for slab creation.
  */
 void __init create_kmalloc_caches(void)
 {
     int i;
     enum kmalloc_cache_type type;
 
     /*
      * Including KMALLOC_CGROUP if CONFIG_MEMCG defined
      */
     for (type = KMALLOC_NORMAL; type < NR_KMALLOC_TYPES; type++) {
         /* Caches that are NOT of the two-to-the-power-of size. */
         if (KMALLOC_MIN_SIZE <= 32)
             new_kmalloc_cache(1, type);
         if (KMALLOC_MIN_SIZE <= 64)
             new_kmalloc_cache(2, type);
 
         /* Caches that are of the two-to-the-power-of size. */
         for (i = KMALLOC_SHIFT_LOW; i <= KMALLOC_SHIFT_HIGH; i++)
             new_kmalloc_cache(i, type);
     }
 #ifdef CONFIG_RANDOM_KMALLOC_CACHES
     random_kmalloc_seed = get_random_u64();
 #endif
 
     /* Kmalloc array is now usable */
     slab_state = UP;
 
     if (IS_ENABLED(CONFIG_SLAB_BUCKETS))
         kmem_buckets_cache = kmem_cache_create("kmalloc_buckets",
                                sizeof(kmem_buckets),
                                0, SLAB_NO_MERGE, NULL);
 }
 
 /**
  * __ksize -- Report full size of underlying allocation
  * @object: pointer to the object
  *
  * This should only be used internally to query the true size of allocations.
  * It is not meant to be a way to discover the usable size of an allocation
  * after the fact. Instead, use kmalloc_size_roundup(). Using memory beyond
  * the originally requested allocation size may trigger KASAN, UBSAN_BOUNDS,
  * and/or FORTIFY_SOURCE.
  *
  * Return: size of the actual memory used by @object in bytes
  */
 size_t __ksize(const void *object)
 {
     struct folio *folio;
 
     if (unlikely(object == ZERO_SIZE_PTR))
         return 0;
 
     folio = virt_to_folio(object);
 
     if (unlikely(!folio_test_slab(folio))) {
         if (WARN_ON(folio_size(folio) <= KMALLOC_MAX_CACHE_SIZE))
             return 0;
         if (WARN_ON(object != folio_address(folio)))
             return 0;
         return folio_size(folio);
     }
 
 #ifdef CONFIG_SLUB_DEBUG
     skip_orig_size_check(folio_slab(folio)->slab_cache, object);
 #endif
 
     return slab_ksize(folio_slab(folio)->slab_cache);
 }
 
 gfp_t kmalloc_fix_flags(gfp_t flags)
 {
     gfp_t invalid_mask = flags & GFP_SLAB_BUG_MASK;
 
     flags &= ~GFP_SLAB_BUG_MASK;
     pr_warn("Unexpected gfp: %#x (%pGg). Fixing up to gfp: %#x (%pGg). Fix your code!\n",
             invalid_mask, &invalid_mask, flags, &flags);
     dump_stack();
 
     return flags;
 }
 
 #ifdef CONFIG_SLAB_FREELIST_RANDOM
 /* Randomize a generic freelist */
 static void freelist_randomize(unsigned int *list,
                    unsigned int count)
 {
     unsigned int rand;
     unsigned int i;
 
     for (i = 0; i < count; i++)
         list[i] = i;
 
     /* Fisher-Yates shuffle */
     for (i = count - 1; i > 0; i--) {
         rand = get_random_u32_below(i + 1);
         swap(list[i], list[rand]);
     }
 }
 
 /* Create a random sequence per cache */
 int cache_random_seq_create(struct kmem_cache *cachep, unsigned int count,
                     gfp_t gfp)
 {
 
     if (count < 2 || cachep->random_seq)
         return 0;
 
     cachep->random_seq = kcalloc(count, sizeof(unsigned int), gfp);
     if (!cachep->random_seq)
         return -ENOMEM;
 
     freelist_randomize(cachep->random_seq, count);
     return 0;
 }
 
 /* Destroy the per-cache random freelist sequence */
 void cache_random_seq_destroy(struct kmem_cache *cachep)
 {
     kfree(cachep->random_seq);
     cachep->random_seq = NULL;
 }
 #endif /* CONFIG_SLAB_FREELIST_RANDOM */
 

 static __always_inline __realloc_size(2) void *
 __do_krealloc(const void *p, size_t new_size, gfp_t flags)
 {
     void *ret;
     size_t ks;
 
     /* Check for double-free before calling ksize. */
     if (likely(!ZERO_OR_NULL_PTR(p))) {
         if (!kasan_check_byte(p))
             return NULL;
         ks = ksize(p);
     } else
         ks = 0;
 
     /* If the object still fits, repoison it precisely. */
     if (ks >= new_size) {
         p = kasan_krealloc((void *)p, new_size, flags);
         return (void *)p;
     }
 
     ret = kmalloc_node_track_caller_noprof(new_size, flags, NUMA_NO_NODE, _RET_IP_);
     if (ret && p) {
         /* Disable KASAN checks as the object's redzone is accessed. */
         kasan_disable_current();
         memcpy(ret, kasan_reset_tag(p), ks);
         kasan_enable_current();
     }
 
     return ret;
 }
 
 /**
  * krealloc - reallocate memory. The contents will remain unchanged.
  * @p: object to reallocate memory for.
  * @new_size: how many bytes of memory are required.
  * @flags: the type of memory to allocate.
  *
  * The contents of the object pointed to are preserved up to the
  * lesser of the new and old sizes (__GFP_ZERO flag is effectively ignored).
  * If @p is %NULL, krealloc() behaves exactly like kmalloc().  If @new_size
  * is 0 and @p is not a %NULL pointer, the object pointed to is freed.
  *
  * Return: pointer to the allocated memory or %NULL in case of error
  */
 void *krealloc_noprof(const void *p, size_t new_size, gfp_t flags)
 {
     void *ret;
 
     if (unlikely(!new_size)) {
         kfree(p);
         return ZERO_SIZE_PTR;
     }
 
     ret = __do_krealloc(p, new_size, flags);
     if (ret && kasan_reset_tag(p) != kasan_reset_tag(ret))
         kfree(p);
 
     return ret;
 }
 EXPORT_SYMBOL(krealloc_noprof);
 
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
 }
 
 size_t ksize(const void *objp)
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
     if (unlikely(ZERO_OR_NULL_PTR(objp)) || !kasan_check_byte(objp))
         return 0;
 
     return kfence_ksize(objp) ?: __ksize(objp);
 }*/

#[no_mangle]
pub extern "C" fn kmem_cache_size(s: *const kmem_cache) -> u32 {
    unsafe {
        (*s).object_size
    }
}