#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Container Container;

uintptr_t add(uintptr_t left, uintptr_t right);

struct Container *new_container(const char *s);

const char *get_info(const struct Container *c);

void destroy(char *c);
