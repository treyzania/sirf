#include "libsirf.h"

sirf_rheader* sirf_find_record(sird* data, char* name) {

	sirf_rheader* at = (sirf_rheader*) (data + 1);
	for (uint32_t i = 0; i < data->recordcnt; i++) {

		char* rname = (char*) (at + sizeof(sirf_rheader));
		if (strcmp(name, rname) == 0) { // 0 for equality.
			return at;
		} else {
			at += sirf_total_size(at); // 1 for null terminator.
		}

	}

	return NULL;

}

void* sirf_get_data(sird* data, char* name) {

	sirf_rheader* rec = sirf_find_record(data, name);
	return ((void*) rec) + rec->namelen + 1; // 1 for null terminator.

}

void* sirf_index(sird* data, uint64_t index) {

	if (index >= data->recordcnt) {
		return NULL;
	}

	sirf_rheader* at = (sirf_rheader*) (data + 1);
	for (uint64_t i; i <= index; i++) { // le because we actually want what's right after it.
		at += sirf_total_size(at);
	}

	return (void*) at; // Cast because we've actually overshot it and are here.

}
