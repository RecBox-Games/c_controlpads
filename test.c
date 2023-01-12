#include <stdio.h>
#include <stdint.h>
#include <unistd.h>
#include <stdbool.h>

typedef struct c_string_vec {
  char **ptr;
  uint64_t len;
  uint64_t cap;
} c_string_vec;

typedef uint64_t Err;
static const Err SUCCESS = 0;
static const Err FAILURE = 1;

extern void free_strvec(c_string_vec vec);
extern Err clients_changed(bool *did_change);
extern Err get_client_handles(c_string_vec *client_handles);

// control pad check
#define CP_CHECK(x) do {                                                \
        uint64_t retval = (x);                                          \
        if (retval != 0) {                                              \
            fprintf(stderr, "controlpads error: %s returned %ld as %s:%d", \
                    #x, retval, __FILE__, __LINE__);                    \
        }                                                               \
    } while (0)
    

int main() {
    uint64_t err;
    bool did_change;
    c_string_vec handles;
    while (1) {
        CP_CHECK(clients_changed(&did_change));
        if (did_change) {
            printf("Handles changed:\n");
            CP_CHECK(get_client_handles(&handles));
            for (int i = 0; i < handles.len; i++) {
                printf("  %s\n", handles.ptr[i]);
            }
        }
        sleep(0.1);
    }

    return 0;
}
