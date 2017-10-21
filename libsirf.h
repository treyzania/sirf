#include <stdint.h>
#include <string.h>

#define SIRF_ATTRS __attribute__((__packed__)) __attribute__((__scalar_storage_order__("big-endian")))

struct SIRF_ATTRS sirf_header_t {
	uint32_t magic;
	uint32_t recordcnt;
} SIRF_ATTRS;

typedef struct sirf_header_t sird;

struct sirf_record_header_t {
	uint64_t size;
	uint16_t namelen;
} SIRF_ATTRS;

typedef struct sirf_record_header_t sirf_rheader;

sirf_rheader* sirf_find_record(sird* data, char* name);
void* sirf_record(sird* data, char* name);
void* sirf_index(sird* data, uint64_t index);

#define sirf_total_size(recptr) (sizeof(sirf_rheader) + recptr->size + recptr->namelen + 1)
