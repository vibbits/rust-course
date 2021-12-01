#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

typedef struct my_resource_t {
    int32_t val;
} my_resource_t;

my_resource_t * create_resource(const int32_t a) {
    printf("Creating resource %d\n", a);
    my_resource_t * const r = calloc(1, sizeof(my_resource_t));
    r->val = a;    
    return r;
}

int32_t get_resource_value(const my_resource_t * const r) {
    return r->val;
}

void set_resource_value(my_resource_t * const r, const int32_t a) {
    r->val = a;
}

void dispose_resource(my_resource_t * const r) {
    printf("Deleting resource %d\n", r->val);
    free(r);
} 