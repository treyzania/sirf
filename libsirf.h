#include <stdint.h>
#include <string.h>

struct __attribute__((__packed__)) sirf_header_t {
	uint32_t magic;
	uint32_t recordcnt;
};

typedef struct sirf_header_t sird;

struct __attribute__((__packed__)) sirf_record_header_t {
	uint64_t size;
	uint16_t namelen;
};

typedef struct sirf_record_header_t sirf_rheader;

sirf_rheader* sirf_find_record(sird* data, char* name);
void* sirf_record(sird* data, char* name);
void* sirf_index(sird* data, uint64_t index);
