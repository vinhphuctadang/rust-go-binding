#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Container Container;

typedef struct Product Product;

uintptr_t add(uintptr_t left, uintptr_t right);

struct Container *new_container(const char *prod_name, uint32_t quantity);

const char *get_name(const struct Container *c);

void destroy(char *s);

uint32_t get_quantity(const struct Container *c);

struct Product *new_product(uint64_t date, struct Container *product_container);

const struct Container *get_container(const struct Product *product);
