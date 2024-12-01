// SPDX-License-Identifier: GPL-2.0
/*
 * Slab allocator functions that are independent of the allocator strategy
 *
 * (C) 2012 Christoph Lameter <cl@linux.com>
 */
#include <linux/slab.h>

#include <linux/mm.h>
#include <linux/poison.h>
#include <linux/interrupt.h>
#include <linux/memory.h>
#include <linux/cache.h>
#include <linux/compiler.h>
#include <linux/kfence.h>
#include <linux/module.h>
#include <linux/cpu.h>
#include <linux/uaccess.h>
#include <linux/seq_file.h>
#include <linux/dma-mapping.h>
#include <linux/swiotlb.h>
#include <linux/proc_fs.h>
#include <linux/debugfs.h>
#include <linux/kmemleak.h>
#include <linux/kasan.h>
#include <asm/cacheflush.h>
#include <asm/tlbflush.h>
#include <asm/page.h>
#include <linux/memcontrol.h>
#include <linux/stackdepot.h>

#include "internal.h"
#include "slab.h"

#define CREATE_TRACE_POINTS
#include <trace/events/kmem.h>

enum slab_state slab_state;
LIST_HEAD(slab_caches);
DEFINE_MUTEX(slab_mutex);
struct kmem_cache *kmem_cache;

extern LIST_HEAD(slab_caches_to_rcu_destroy);
static void slab_caches_to_rcu_destroy_workfn(struct work_struct *work);
extern DECLARE_WORK(slab_caches_to_rcu_destroy_work,
		    slab_caches_to_rcu_destroy_workfn);

/*
 * Set of flags that will prevent slab merging
 */
#define SLAB_NEVER_MERGE (SLAB_RED_ZONE | SLAB_POISON | SLAB_STORE_USER | \
		SLAB_TRACE | SLAB_TYPESAFE_BY_RCU | SLAB_NOLEAKTRACE | \
		SLAB_FAILSLAB | SLAB_NO_MERGE)

#define SLAB_MERGE_SAME (SLAB_RECLAIM_ACCOUNT | SLAB_CACHE_DMA | \
			 SLAB_CACHE_DMA32 | SLAB_ACCOUNT)

/*
 * Merge control. If this is set then no merging of slab caches will occur.
 */
static bool slab_nomerge = !IS_ENABLED(CONFIG_SLAB_MERGE_DEFAULT);

static int __init setup_slab_nomerge(char *str)
{
	slab_nomerge = true;
	return 1;
}

static int __init setup_slab_merge(char *str)
{
	slab_nomerge = false;
	return 1;
}

__setup_param("slub_nomerge", slub_nomerge, setup_slab_nomerge, 0);
__setup_param("slub_merge", slub_merge, setup_slab_merge, 0);

__setup("slab_nomerge", setup_slab_nomerge);
__setup("slab_merge", setup_slab_merge);

/*
 * Determine the size of a slab object
 */
/*
unsigned int kmem_cache_size(struct kmem_cache *s)
{
	return s->object_size;
}
*/
EXPORT_SYMBOL(kmem_cache_size);

#ifdef CONFIG_DEBUG_VM
static int kmem_cache_sanity_check(const char *name, unsigned int size)
{
	if (!name || in_interrupt() || size > KMALLOC_MAX_SIZE) {
		pr_err("kmem_cache_create(%s) integrity check failed\n", name);
		return -EINVAL;
	}

	WARN_ON(strchr(name, ' '));	/* It confuses parsers */
	return 0;
}
#else
static inline int kmem_cache_sanity_check(const char *name, unsigned int size)
{
	return 0;
}
#endif


/*
From Ryan: This is kind of a jank work around for some symbol linking, don't
worry about it too much
*/
bool schedule_work_link(struct work_struct *work)
{
	return schedule_work(work);
}
EXPORT_SYMBOL(schedule_work_link);

void kasan_cache_shutdown_link(struct kmem_cache *cache) {
	kasan_cache_shutdown(cache);
}
EXPORT_SYMBOL(kasan_cache_shutdown_link);

void kasan_cache_shrink_link(struct kmem_cache *cache) {
	kasan_cache_shrink(cache);
}
EXPORT_SYMBOL(kasan_cache_shrink_link);

struct kmem_cache *kmem_cache_create_kernel_link(void) {
	return kmem_cache_zalloc(kmem_cache, GFP_KERNEL);
}
EXPORT_SYMBOL(kmem_cache_create_kernel_link);

struct kmem_cache *kmem_cache_create_nowait_link(void) {
	return kmem_cache_zalloc(kmem_cache, GFP_NOWAIT);
}
EXPORT_SYMBOL(kmem_cache_create_nowait_link);

struct kmem_cache *
kmalloc_slab_link(size_t size, kmem_buckets *b, gfp_t flags, unsigned long caller) {
	return kmalloc_slab(size, b, flags, caller);
}
EXPORT_SYMBOL(kmalloc_slab_link);

int get_order_link(unsigned long size)
{
	return get_order(size);
}
EXPORT_SYMBOL(get_order_link);

unsigned long dma_get_cache_alignment_link(void) {
	return dma_get_cache_alignment();
}
EXPORT_SYMBOL(dma_get_cache_alignment_link);

unsigned long config_dma_bounce_unaligned_link(void) {
	return IS_ENABLED(CONFIG_DMA_BOUNCE_UNALIGNED_KMALLOC) &&
	    is_swiotlb_allocated();
}
EXPORT_SYMBOL(config_dma_bounce_unaligned_link);

unsigned long arch_slab_minalign_link(void) {
	return arch_slab_minalign();
}
EXPORT_SYMBOL(arch_slab_minalign_link);

bool mem_cgroup_kmem_disabled_link(void) {
	return mem_cgroup_kmem_disabled();
}
EXPORT_SYMBOL(mem_cgroup_kmem_disabled_link);

 struct folio *virt_to_folio_link(const void *x) {
	return virt_to_folio(x);
}
EXPORT_SYMBOL(virt_to_folio_link);

 void *folio_address_link(const struct folio *folio) {
	return folio_address(folio);
}
EXPORT_SYMBOL(folio_address_link);

 size_t folio_size_link(const struct folio *folio) {
	return folio_size(folio);
}
EXPORT_SYMBOL(folio_size_link);

 bool folio_test_slab_link(const struct folio *folio) {
	return folio_test_slab(folio);
}
EXPORT_SYMBOL(folio_test_slab_link);
void skip_orig_size_check_link(struct kmem_cache *s, const void *object) {
	return skip_orig_size_check(s, object);
}
EXPORT_SYMBOL(skip_orig_size_check_link);

void *kcalloc_link(unsigned long n, unsigned long size, gfp_t flags) {
	return kmalloc_array(n, size, (flags) | __GFP_ZERO);
}		
EXPORT_SYMBOL(kcalloc_link);

unsigned long get_random_u32_below_link(unsigned long i) {
	return get_random_u32_below(i);
}
EXPORT_SYMBOL(get_random_u32_below_link);
EXPORT_SYMBOL(calculate_alignment);

 
extern int slab_unmergeable(struct kmem_cache *s);


extern struct kmem_cache *find_mergeable(unsigned int size, unsigned int align,
 		slab_flags_t flags, const char *name, void (*ctor)(void *));
// struct kmem_cache *find_mergeable(unsigned int size, unsigned int align,
// 		slab_flags_t flags, const char *name, void (*ctor)(void *))
// {
// 	struct kmem_cache *s;

// 	if (slab_nomerge)
// 		return NULL;

// 	if (ctor)
// 		return NULL;

// 	size = ALIGN(size, sizeof(void *));
// 	align = calculate_alignment(flags, align, size);
// 	size = ALIGN(size, align);
// flags = kmem_cache_flags(flags, name);

// 	if (flags & SLAB_NEVER_MERGE)
// 		return NULL;

// 	list_for_each_entry_reverse(s, &slab_caches, list) {
// 		if (slab_unmergeable(s))
// 			continue;

// 		if (size > s->size)
// 			continue;

// 		if ((flags & SLAB_MERGE_SAME) != (s->flags & SLAB_MERGE_SAME))
// 			continue;
// 		/*
// 		 * Check if alignment is compatible.
// 		 * Courtesy of Adrian Drzewiecki
// 		 */
// 		if ((s->size & ~(align - 1)) != s->size)
// 			continue;

// 		if (s->size - size >= sizeof(void *))
// 			continue;

// 		return s;
// 	}
// 	return NULL;
// }

extern struct kmem_cache *create_cache(const char *name,
		unsigned int object_size, unsigned int align,
		slab_flags_t flags, unsigned int useroffset,
		unsigned int usersize, void (*ctor)(void *),
		struct kmem_cache *root_cache);

extern struct kmem_cache *
kmem_cache_create_usercopy(const char *name,
		  unsigned int size, unsigned int align,
		  slab_flags_t flags,
		  unsigned int useroffset, unsigned int usersize,
		  void (*ctor)(void *));

EXPORT_SYMBOL(kmem_cache_create_usercopy);

extern struct kmem_cache *
kmem_cache_create(const char *name, unsigned int size, unsigned int align,
		slab_flags_t flags, void (*ctor)(void *));
EXPORT_SYMBOL(kmem_cache_create);

struct kmem_cache *kmem_buckets_cache __ro_after_init;

extern kmem_buckets *kmem_buckets_create(const char *name, slab_flags_t flags,
				  unsigned int useroffset,
				  unsigned int usersize,
				  void (*ctor)(void *));

EXPORT_SYMBOL(kmem_buckets_create);

#ifdef SLAB_SUPPORTS_SYSFS

extern void kmem_cache_release(struct kmem_cache *s);

#else
static void kmem_cache_release(struct kmem_cache *s)
{
	slab_kmem_cache_release(s);
}
#endif

extern void slab_caches_to_rcu_destroy_workfn(struct work_struct *work);
extern int shutdown_cache(struct kmem_cache *s);
extern void slab_kmem_cache_release(struct kmem_cache *s);
extern void kmem_cache_destroy(struct kmem_cache *s);
EXPORT_SYMBOL(kmem_cache_destroy);

extern int kmem_cache_shrink(struct kmem_cache *cachep);
EXPORT_SYMBOL(kmem_cache_shrink);

extern bool slab_is_available(void);

#ifdef CONFIG_PRINTK
extern void kmem_obj_info(struct kmem_obj_info *kpp, void *object, struct slab *slab);


/**
 * kmem_dump_obj - Print available slab provenance information
 * @object: slab object for which to find provenance information.
 *
 * This function uses pr_cont(), so that the caller is expected to have
 * printed out whatever preamble is appropriate.  The provenance information
 * depends on the type of object and on how much debugging is enabled.
 * For a slab-cache object, the fact that it is a slab object is printed,
 * and, if available, the slab name, return address, and stack trace from
 * the allocation and last free path of that object.
 *
 * Return: %true if the pointer is to a not-yet-freed object from
 * kmalloc() or kmem_cache_alloc(), either %true or %false if the pointer
 * is to an already-freed object, and %false otherwise.
 */
bool kmem_dump_obj(void *object)
{
	char *cp = IS_ENABLED(CONFIG_MMU) ? "" : "/vmalloc";
	int i;
	struct slab *slab;
	unsigned long ptroffset;
	struct kmem_obj_info kp = { };

	/* Some arches consider ZERO_SIZE_PTR to be a valid address. */
	if (object < (void *)PAGE_SIZE || !virt_addr_valid(object))
		return false;
	slab = virt_to_slab(object);
	if (!slab)
		return false;

	kmem_obj_info(&kp, object, slab);
	if (kp.kp_slab_cache)
		pr_cont(" slab%s %s", cp, kp.kp_slab_cache->name);
	else
		pr_cont(" slab%s", cp);
	if (is_kfence_address(object))
		pr_cont(" (kfence)");
	if (kp.kp_objp)
		pr_cont(" start %px", kp.kp_objp);
	if (kp.kp_data_offset)
		pr_cont(" data offset %lu", kp.kp_data_offset);
	if (kp.kp_objp) {
		ptroffset = ((char *)object - (char *)kp.kp_objp) - kp.kp_data_offset;
		pr_cont(" pointer offset %lu", ptroffset);
	}
	if (kp.kp_slab_cache && kp.kp_slab_cache->object_size)
		pr_cont(" size %u", kp.kp_slab_cache->object_size);
	if (kp.kp_ret)
		pr_cont(" allocated at %pS\n", kp.kp_ret);
	else
		pr_cont("\n");
	for (i = 0; i < ARRAY_SIZE(kp.kp_stack); i++) {
		if (!kp.kp_stack[i])
			break;
		pr_info("    %pS\n", kp.kp_stack[i]);
	}

	if (kp.kp_free_stack[0])
		pr_cont(" Free path:\n");

	for (i = 0; i < ARRAY_SIZE(kp.kp_free_stack); i++) {
		if (!kp.kp_free_stack[i])
			break;
		pr_info("    %pS\n", kp.kp_free_stack[i]);
	}

	return true;
}
EXPORT_SYMBOL_GPL(kmem_dump_obj);
#endif

/* Create a cache during boot when no slab services are available yet */
extern void create_boot_cache(struct kmem_cache *s, const char *name,
		unsigned int size, slab_flags_t flags,
		unsigned int useroffset, unsigned int usersize);
extern struct kmem_cache * create_kmalloc_cache(const char *name,
						      unsigned int size,
						      slab_flags_t flags);

kmem_buckets kmalloc_caches[NR_KMALLOC_TYPES] __ro_after_init =
{ /* initialization for https://llvm.org/pr42570 */ };
EXPORT_SYMBOL(kmalloc_caches);

#ifdef CONFIG_RANDOM_KMALLOC_CACHES
unsigned long random_kmalloc_seed __ro_after_init;
EXPORT_SYMBOL(random_kmalloc_seed);
#endif

/*
 * Conversion table for small slabs sizes / 8 to the index in the
 * kmalloc array. This is necessary for slabs < 192 since we have non power
 * of two cache sizes there. The size of larger slabs can be determined using
 * fls.
 */
u8 kmalloc_size_index[24] __ro_after_init = {
	3,	/* 8 */
	4,	/* 16 */
	5,	/* 24 */
	5,	/* 32 */
	6,	/* 40 */
	6,	/* 48 */
	6,	/* 56 */
	6,	/* 64 */
	1,	/* 72 */
	1,	/* 80 */
	1,	/* 88 */
	1,	/* 96 */
	7,	/* 104 */
	7,	/* 112 */
	7,	/* 120 */
	7,	/* 128 */
	2,	/* 136 */
	2,	/* 144 */
	2,	/* 152 */
	2,	/* 160 */
	2,	/* 168 */
	2,	/* 176 */
	2,	/* 184 */
	2	/* 192 */
};

extern size_t kmalloc_size_roundup(size_t size);
EXPORT_SYMBOL(kmalloc_size_roundup);

#ifdef CONFIG_ZONE_DMA
#define KMALLOC_DMA_NAME(sz)	.name[KMALLOC_DMA] = "dma-kmalloc-" #sz,
#else
#define KMALLOC_DMA_NAME(sz)
#endif

#ifdef CONFIG_MEMCG
#define KMALLOC_CGROUP_NAME(sz)	.name[KMALLOC_CGROUP] = "kmalloc-cg-" #sz,
#else
#define KMALLOC_CGROUP_NAME(sz)
#endif

#ifndef CONFIG_SLUB_TINY
#define KMALLOC_RCL_NAME(sz)	.name[KMALLOC_RECLAIM] = "kmalloc-rcl-" #sz,
#else
#define KMALLOC_RCL_NAME(sz)
#endif

#ifdef CONFIG_RANDOM_KMALLOC_CACHES
#define __KMALLOC_RANDOM_CONCAT(a, b) a ## b
#define KMALLOC_RANDOM_NAME(N, sz) __KMALLOC_RANDOM_CONCAT(KMA_RAND_, N)(sz)
#define KMA_RAND_1(sz)                  .name[KMALLOC_RANDOM_START +  1] = "kmalloc-rnd-01-" #sz,
#define KMA_RAND_2(sz)  KMA_RAND_1(sz)  .name[KMALLOC_RANDOM_START +  2] = "kmalloc-rnd-02-" #sz,
#define KMA_RAND_3(sz)  KMA_RAND_2(sz)  .name[KMALLOC_RANDOM_START +  3] = "kmalloc-rnd-03-" #sz,
#define KMA_RAND_4(sz)  KMA_RAND_3(sz)  .name[KMALLOC_RANDOM_START +  4] = "kmalloc-rnd-04-" #sz,
#define KMA_RAND_5(sz)  KMA_RAND_4(sz)  .name[KMALLOC_RANDOM_START +  5] = "kmalloc-rnd-05-" #sz,
#define KMA_RAND_6(sz)  KMA_RAND_5(sz)  .name[KMALLOC_RANDOM_START +  6] = "kmalloc-rnd-06-" #sz,
#define KMA_RAND_7(sz)  KMA_RAND_6(sz)  .name[KMALLOC_RANDOM_START +  7] = "kmalloc-rnd-07-" #sz,
#define KMA_RAND_8(sz)  KMA_RAND_7(sz)  .name[KMALLOC_RANDOM_START +  8] = "kmalloc-rnd-08-" #sz,
#define KMA_RAND_9(sz)  KMA_RAND_8(sz)  .name[KMALLOC_RANDOM_START +  9] = "kmalloc-rnd-09-" #sz,
#define KMA_RAND_10(sz) KMA_RAND_9(sz)  .name[KMALLOC_RANDOM_START + 10] = "kmalloc-rnd-10-" #sz,
#define KMA_RAND_11(sz) KMA_RAND_10(sz) .name[KMALLOC_RANDOM_START + 11] = "kmalloc-rnd-11-" #sz,
#define KMA_RAND_12(sz) KMA_RAND_11(sz) .name[KMALLOC_RANDOM_START + 12] = "kmalloc-rnd-12-" #sz,
#define KMA_RAND_13(sz) KMA_RAND_12(sz) .name[KMALLOC_RANDOM_START + 13] = "kmalloc-rnd-13-" #sz,
#define KMA_RAND_14(sz) KMA_RAND_13(sz) .name[KMALLOC_RANDOM_START + 14] = "kmalloc-rnd-14-" #sz,
#define KMA_RAND_15(sz) KMA_RAND_14(sz) .name[KMALLOC_RANDOM_START + 15] = "kmalloc-rnd-15-" #sz,
#else // CONFIG_RANDOM_KMALLOC_CACHES
#define KMALLOC_RANDOM_NAME(N, sz)
#endif

#define INIT_KMALLOC_INFO(__size, __short_size)			\
{								\
	.name[KMALLOC_NORMAL]  = "kmalloc-" #__short_size,	\
	KMALLOC_RCL_NAME(__short_size)				\
	KMALLOC_CGROUP_NAME(__short_size)			\
	KMALLOC_DMA_NAME(__short_size)				\
	KMALLOC_RANDOM_NAME(RANDOM_KMALLOC_CACHES_NR, __short_size)	\
	.size = __size,						\
}

/*
 * kmalloc_info[] is to make slab_debug=,kmalloc-xx option work at boot time.
 * kmalloc_index() supports up to 2^21=2MB, so the final entry of the table is
 * kmalloc-2M.
 */
extern const struct kmalloc_info_struct kmalloc_info[] __initconst = {
	INIT_KMALLOC_INFO(0, 0),
	INIT_KMALLOC_INFO(96, 96),
	INIT_KMALLOC_INFO(192, 192),
	INIT_KMALLOC_INFO(8, 8),
	INIT_KMALLOC_INFO(16, 16),
	INIT_KMALLOC_INFO(32, 32),
	INIT_KMALLOC_INFO(64, 64),
	INIT_KMALLOC_INFO(128, 128),
	INIT_KMALLOC_INFO(256, 256),
	INIT_KMALLOC_INFO(512, 512),
	INIT_KMALLOC_INFO(1024, 1k),
	INIT_KMALLOC_INFO(2048, 2k),
	INIT_KMALLOC_INFO(4096, 4k),
	INIT_KMALLOC_INFO(8192, 8k),
	INIT_KMALLOC_INFO(16384, 16k),
	INIT_KMALLOC_INFO(32768, 32k),
	INIT_KMALLOC_INFO(65536, 64k),
	INIT_KMALLOC_INFO(131072, 128k),
	INIT_KMALLOC_INFO(262144, 256k),
	INIT_KMALLOC_INFO(524288, 512k),
	INIT_KMALLOC_INFO(1048576, 1M),
	INIT_KMALLOC_INFO(2097152, 2M)
};


extern void __init setup_kmalloc_cache_index_table(void);
extern unsigned int __kmalloc_minalign(void);
extern void __init new_kmalloc_cache(int idx, enum kmalloc_cache_type type);
extern void __init create_kmalloc_caches(void);
extern size_t __ksize(const void *object);
extern gfp_t kmalloc_fix_flags(gfp_t flags);


#ifdef CONFIG_SLAB_FREELIST_RANDOM
/* Randomize a generic freelist */
extern void freelist_randomize(unsigned int *list,
			       unsigned int count);
// {
// 	unsigned int rand;
// 	unsigned int i;

// 	for (i = 0; i < count; i++)
// 		list[i] = i;

// 	/* Fisher-Yates shuffle */
// 	for (i = count - 1; i > 0; i--) {
// 		rand = get_random_u32_below(i + 1);
// 		swap(list[i], list[rand]);
// 	}
// }

/* Create a random sequence per cache */
extern int cache_random_seq_create(struct kmem_cache *cachep, unsigned int count,
				    gfp_t gfp);
// {

// 	if (count < 2 || cachep->random_seq)
// 		return 0;

// 	cachep->random_seq = kcalloc(count, sizeof(unsigned int), gfp);
// 	if (!cachep->random_seq)
// 		return -ENOMEM;

// 	freelist_randomize(cachep->random_seq, count);
// 	return 0;
// }

/* Destroy the per-cache random freelist sequence */
void cache_random_seq_destroy(struct kmem_cache *cachep)
{
	kfree(cachep->random_seq);
	cachep->random_seq = NULL;
}
#endif /* CONFIG_SLAB_FREELIST_RANDOM */

#ifdef CONFIG_SLUB_DEBUG
#define SLABINFO_RIGHTS (0400)

static void print_slabinfo_header(struct seq_file *m)
{
	/*
	 * Output format version, so at least we can change it
	 * without _too_ many complaints.
	 */
	seq_puts(m, "slabinfo - version: 2.1\n");
	seq_puts(m, "# name            <active_objs> <num_objs> <objsize> <objperslab> <pagesperslab>");
	seq_puts(m, " : tunables <limit> <batchcount> <sharedfactor>");
	seq_puts(m, " : slabdata <active_slabs> <num_slabs> <sharedavail>");
	seq_putc(m, '\n');
}

static void *slab_start(struct seq_file *m, loff_t *pos)
{
	mutex_lock(&slab_mutex);
	return seq_list_start(&slab_caches, *pos);
}

static void *slab_next(struct seq_file *m, void *p, loff_t *pos)
{
	return seq_list_next(p, &slab_caches, pos);
}

static void slab_stop(struct seq_file *m, void *p)
{
	mutex_unlock(&slab_mutex);
}

static void cache_show(struct kmem_cache *s, struct seq_file *m)
{
	struct slabinfo sinfo;

	memset(&sinfo, 0, sizeof(sinfo));
	get_slabinfo(s, &sinfo);

	seq_printf(m, "%-17s %6lu %6lu %6u %4u %4d",
		   s->name, sinfo.active_objs, sinfo.num_objs, s->size,
		   sinfo.objects_per_slab, (1 << sinfo.cache_order));

	seq_printf(m, " : tunables %4u %4u %4u",
		   sinfo.limit, sinfo.batchcount, sinfo.shared);
	seq_printf(m, " : slabdata %6lu %6lu %6lu",
		   sinfo.active_slabs, sinfo.num_slabs, sinfo.shared_avail);
	seq_putc(m, '\n');
}

static int slab_show(struct seq_file *m, void *p)
{
	struct kmem_cache *s = list_entry(p, struct kmem_cache, list);

	if (p == slab_caches.next)
		print_slabinfo_header(m);
	cache_show(s, m);
	return 0;
}

void dump_unreclaimable_slab(void)
{
	struct kmem_cache *s;
	struct slabinfo sinfo;

	/*
	 * Here acquiring slab_mutex is risky since we don't prefer to get
	 * sleep in oom path. But, without mutex hold, it may introduce a
	 * risk of crash.
	 * Use mutex_trylock to protect the list traverse, dump nothing
	 * without acquiring the mutex.
	 */
	if (!mutex_trylock(&slab_mutex)) {
		pr_warn("excessive unreclaimable slab but cannot dump stats\n");
		return;
	}

	pr_info("Unreclaimable slab info:\n");
	pr_info("Name                      Used          Total\n");

	list_for_each_entry(s, &slab_caches, list) {
		if (s->flags & SLAB_RECLAIM_ACCOUNT)
			continue;

		get_slabinfo(s, &sinfo);

		if (sinfo.num_objs > 0)
			pr_info("%-17s %10luKB %10luKB\n", s->name,
				(sinfo.active_objs * s->size) / 1024,
				(sinfo.num_objs * s->size) / 1024);
	}
	mutex_unlock(&slab_mutex);
}

/*
 * slabinfo_op - iterator that generates /proc/slabinfo
 *
 * Output layout:
 * cache-name
 * num-active-objs
 * total-objs
 * object size
 * num-active-slabs
 * total-slabs
 * num-pages-per-slab
 * + further values on SMP and with statistics enabled
 */
static const struct seq_operations slabinfo_op = {
	.start = slab_start,
	.next = slab_next,
	.stop = slab_stop,
	.show = slab_show,
};

static int slabinfo_open(struct inode *inode, struct file *file)
{
	return seq_open(file, &slabinfo_op);
}

static const struct proc_ops slabinfo_proc_ops = {
	.proc_flags	= PROC_ENTRY_PERMANENT,
	.proc_open	= slabinfo_open,
	.proc_read	= seq_read,
	.proc_lseek	= seq_lseek,
	.proc_release	= seq_release,
};

static int __init slab_proc_init(void)
{
	proc_create("slabinfo", SLABINFO_RIGHTS, NULL, &slabinfo_proc_ops);
	return 0;
}
module_init(slab_proc_init);

#endif /* CONFIG_SLUB_DEBUG */

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
EXPORT_SYMBOL(kfree_sensitive);

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
}
EXPORT_SYMBOL(ksize);

/* Tracepoints definitions. */
EXPORT_TRACEPOINT_SYMBOL(kmalloc);
EXPORT_TRACEPOINT_SYMBOL(kmem_cache_alloc);
EXPORT_TRACEPOINT_SYMBOL(kfree);
EXPORT_TRACEPOINT_SYMBOL(kmem_cache_free);

