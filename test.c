// The code in this file (test.c) is licensed under CC0 1.0 Universal (CC0 1.0)
// Public Domain Dedication.
// To view a copy of this license, visit
// http://creativecommons.org/publicdomain/zero/1.0/
//
// You can copy, modify, distribute and perform the work, even for commercial
// purposes, all without asking permission. See the CC0 Public Domain
// Dedication for more details.

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
extern Err clients_changed(bool * did_change);
extern Err get_client_handles(c_string_vec * client_handles);
extern Err send_message(char * client, char * msg);
extern Err get_messages(char * client, c_string_vec * messages);

// control pad check
#define CP_CHECK(x) do {                                                \
        uint64_t retval = (x);                                          \
        if (retval != 0) {                                              \
            fprintf(stderr, "controlpads error: %s returned %ld as %s:%d", \
                    #x, retval, __FILE__, __LINE__);                    \
        }                                                               \
    } while (0)
    

int main() {
    uint64_t err = 0;
    bool did_change = false;
    c_string_vec handles = {0};
    c_string_vec messages = {0};
    char * my_msg = "hello";
    while (1) {

        // changing clients
        CP_CHECK(clients_changed(&did_change));
        if (did_change) {
            printf("Handles changed:\n");
            CP_CHECK(get_client_handles(&handles));
            for (int i = 0; i < handles.len; i++) {
                printf("  %s\n", handles.ptr[i]);
            }
        }

        // print inbound messages
        for (int i = 0; i < handles.len; i++) {
            CP_CHECK(get_messages(handles.ptr[i], &messages));
            if (messages.len > 0) {
                printf("Messages from %s:\n", handles.ptr[i]);
                for (int i = 0; i < messages.len; i++) {
                    printf("  %s\n", messages.ptr[i]);
                }
            }
        }

        // send a message to each client
        for (int i = 0; i < handles.len; i++) {
            CP_CHECK(send_message(handles.ptr[i], my_msg));
        }
        
        sleep(1);
    }

    return 0;
}
