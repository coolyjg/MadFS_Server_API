#include <math.h>
#include <stdlib.h>
#include <stdio.h>
#include <assert.h>
#include <sys/stat.h>
#include <string.h>
#include <time.h>
#include <libpmemobj.h>

#define MAX_CHUNK_LEN 512

POBJ_LAYOUT_BEGIN(data_pool);
    POBJ_LAYOUT_TOID(data_pool, struct data);
POBJ_LAYOUT_END(data_pool);

struct data{
    char pa[100]; //path
    int chunk_id;  //chunk id
    char dt[MAX_CHUNK_LEN];  //data, no more than 100 bytes
};

TOID(struct data) find_data(PMEMobjpool* pop, const char* p, int id){
    TOID(struct data) ret;
    POBJ_FOREACH_TYPE(pop, ret){
        if((D_RO(ret)->chunk_id == id) && (!strcmp(p, D_RO(ret)->pa))){
            return ret;
        }
    }
    return TOID_NULL(struct data);
}

int create_data(PMEMobjpool* pop, const char* p, int id, const char* buf, int size){
    TOID(struct data) ret = find_data(pop, p, id);
    if(!TOID_IS_NULL(ret)){
        POBJ_FREE(&ret);
    }
    printf("Creating ....\n");
    // int len2 = strlen(buf);
    // printf("buffer len: %d\n", len2);
    // puts(buf);
    POBJ_ZNEW(pop, &ret, struct data);
    struct data* ret_dir = D_RW(ret);
    ret_dir->chunk_id = id;
    // int len = strlen(buf);
    pmemobj_memcpy_persist(pop, ret_dir->pa, p, strlen(p));
    pmemobj_memcpy_persist(pop, ret_dir->dt, buf, size);
    pmemobj_persist(pop, ret_dir, sizeof(*ret_dir));
    return size;
}

int write_at_data(PMEMobjpool* pop, const char* p, int id, int offset, char* buf, int size){
    TOID(struct data) ret = find_data(pop, p, id);
    int len = strlen(buf);
    printf("before creating, length is :%d\n", len);
    if(TOID_IS_NULL(ret)){
        printf("not found, now create!\n");
        return create_data(pop, p, id, buf, size);
    }
    pmemobj_memcpy_persist(pop, D_RW(ret)->dt+offset, buf, size);
    pmemobj_persist(pop, D_RW(ret), sizeof(*D_RW(ret)));
    return offset+size;
}

// int write_data(PMEMobjpool* pop, char* p, int id, char* buf){
//     return write_at_data(pop, p, id, 0, buf);
// }

int read_at_data(PMEMobjpool* pop, const char* p, int id, int offset, char* buf){
    TOID(struct data) ret = find_data(pop, p, id);
    if(TOID_IS_NULL(ret)){
        printf("not found when reading\n");
        return 0;
    }
    int len = strlen(D_RW(ret)->dt);
    if(offset>=len) return 0;
    sprintf(buf, "%s", D_RW(ret)->dt+offset);
    return len-offset;
}

int read_data(PMEMobjpool* pop, char* p, int id, char* buf){
    return read_at_data(pop, p, id, 0, buf);
}

void free_chunk(PMEMobjpool* pop, char* p, int id){
    TOID(struct data) ret = find_data(pop, p, id);
    if(TOID_IS_NULL(ret)){
        printf("not found\n");
        return;
    }
    POBJ_FREE(&ret);
}

void free_path(PMEMobjpool* pop, char* p){
    TOID(struct data) ret;
    POBJ_FOREACH_TYPE(pop, ret){
        if(!strcmp(p, D_RO(ret)->pa)){
            POBJ_FREE(&ret);
        }
    }
}

PMEMobjpool* init_data(char* path){
    POBJ_LAYOUT_BEGIN(data_pool);
        POBJ_LAYOUT_TOID(data_pool, struct data);
    POBJ_LAYOUT_END(data_pool);
    static PMEMobjpool* pop = NULL;
    if((pop = pmemobj_create(path, POBJ_LAYOUT_NAME(data_pool), PMEMOBJ_MIN_POOL, 0666))==NULL){
        if((pop = pmemobj_open(path, POBJ_LAYOUT_NAME(data_pool)))== NULL){
            printf("fail to open\n");
            return NULL;
        }
    }
    return pop;
}

void fin(PMEMobjpool* pop){
    pmemobj_close(pop);
}


// void print_content(PMEMobjpool* pop, const int id){
//     TOID(struct content) ret = find_content(pop, id);
//     if(TOID_IS_NULL(ret)){
//         printf("not found\n");
//         return;
//     }
//     printf("ino: %d\ncontent: %s\n", D_RW(ret)->ino, D_RW(ret)->cont);
// }

// void print_content_all(PMEMobjpool* pop){
//     TOID(struct content) cnt;
//     POBJ_FOREACH_TYPE(pop, cnt){
//         struct content* cnt_dir = D_RW(cnt);
//         printf("ino: %d\ncontent: %s\n", cnt_dir->ino, cnt_dir->cont);
//     }
// }


POBJ_LAYOUT_BEGIN(metadata_pool); 
    POBJ_LAYOUT_TOID(metadata_pool, struct metadata);
POBJ_LAYOUT_END(metadata_pool);

struct metadata{
    char pa[100];
    size_t atime;
    size_t mtime;
    size_t ctime;
    uint32_t mode;
    uint32_t nlink;
    size_t size;
    size_t chunk_size;
};

TOID(struct metadata) get_metadata(PMEMobjpool* pop, char* p){
    TOID(struct metadata) ret;
    POBJ_FOREACH_TYPE(pop, ret){
        if(!strcmp(p, D_RO(ret)->pa)){
            return ret;
        }
    }
    return TOID_NULL(struct metadata);
}

size_t get_atime(PMEMobjpool* pop, char* p){
    TOID(struct metadata) ret = get_metadata(pop, p);
    return D_RO(ret)->atime;
}

size_t get_mtime(PMEMobjpool* pop, char* p){
    TOID(struct metadata) ret = get_metadata(pop, p);
    return D_RO(ret)->mtime;
}

size_t get_ctime(PMEMobjpool* pop, char* p){
    TOID(struct metadata) ret = get_metadata(pop, p);
    return D_RO(ret)->ctime;
}

uint32_t get_mode(PMEMobjpool* pop, char* p){
    TOID(struct metadata) ret = get_metadata(pop, p);
    return D_RO(ret)->mode;
}

uint32_t get_nlink(PMEMobjpool* pop, char* p){
    TOID(struct metadata) ret = get_metadata(pop, p);
    return D_RO(ret)->nlink;
}

size_t get_size(PMEMobjpool* pop, char* p){
    TOID(struct metadata) ret = get_metadata(pop, p);
    return D_RO(ret)->size;
}

size_t get_chunk_size(PMEMobjpool* pop, char* p){
    TOID(struct metadata) ret = get_metadata(pop, p);
    return D_RO(ret)->chunk_size;
}

void create_metadata(
    PMEMobjpool* pop, 
    char *p,
    size_t atime,
    size_t mtime,
    size_t ctime,
    uint32_t mode,
    uint32_t nlink,
    size_t size,
    size_t chunk_size
){
    TOID(struct metadata) ret = get_metadata(pop, p);
    if(!TOID_IS_NULL(ret)){
        POBJ_FREE(&ret);
    }    
    POBJ_ZNEW(pop, &ret, struct metadata);
    struct metadata* ret_dir = D_RW(ret);
    pmemobj_memcpy_persist(pop, ret_dir->pa, p, strlen(p));
    ret_dir->atime = atime;
    ret_dir->mtime = mtime;
    ret_dir->ctime = ctime;
    ret_dir->mode = mode;
    ret_dir->nlink = nlink;
    ret_dir->size = size;
    ret_dir->chunk_size = chunk_size;
    pmemobj_persist(pop, ret_dir, sizeof(*ret_dir));
}

void remove_metadata(PMEMobjpool* pop, char* p){
    TOID(struct metadata) ret = get_metadata(pop, p);
    if(TOID_IS_NULL(ret)){
        printf("not found\n");
        return;
    }
    POBJ_FREE(&ret);
}

PMEMobjpool* init_mdata(char* path){
    POBJ_LAYOUT_BEGIN(metadata_pool); 
        POBJ_LAYOUT_TOID(metadata_pool, struct metadata);
    POBJ_LAYOUT_END(metadata_pool);
    static PMEMobjpool* pop = NULL;
    if((pop = pmemobj_create(path, POBJ_LAYOUT_NAME(metadata_pool), PMEMOBJ_MIN_POOL, 0666))==NULL){
        if((pop = pmemobj_open(path, POBJ_LAYOUT_NAME(metadata_pool)))== NULL){
            printf("fail to open\n");
            return NULL;
        }
    }
    return pop;
}

