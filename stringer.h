#include <stdlib.h>
#include <stdbool.h>
#include <memory.h>

// easy to use, simple and fast
// ASCII string type implementation.
#ifndef STRINGER
#define STRINGER

typedef struct string {
    char* _str;
    size_t _size;
    size_t _capacity;
}string_t;

// default stringer capacity
#define STRINGER_DEFCAP 2 << 7

// returns a string type with a new allocated
// buff and size 0 and default capacity
// as defined in the header file.
// Or alternatively one could modify the default
// capacity like...
//
// #ifdef STRINGER_DEFCAP
// # undef STRINGER_DEFCAP
// # define STRINGER_DEFCAP 10
// #else
// # define STRINGER_DEFCAP 10
// #endif
string_t str_new();
// creates a new string type from 
// the character pointer. The character pointer
// must be null terminated. if the given pointer
// is null then a string with 0 size and capacity 1
// is created.
string_t str_from_chrptr(const char*);

string_t str_from_c(const char);
// appends to the first argument (dest) from
// the second argument. If one of them are
// NULL or contents of the string (_str) is NULL
// nothing will happen, so that the function
// returns.
void str_append(string_t*, const string_t*);
// Has the same property as str_append on nullability.
// Appends a character pointer to str.
// The character pointer must be null ('\0') terminated.
void str_append_chrptr(string_t*, const char*);
// Has the same property as str_append on nullability.
// appends a single item character.
void str_append_chr(string_t*, const char);
// compares two strings.
// If both strings are the same a true value
// is returned (1) else false is returned (0).
bool str_cmp(string_t*, string_t*);

// returns the size of the string
size_t str_size(const string_t*);
// free up the string
void str_free(string_t*);

// reset stringer
// simple string implementation
#define STRINGER_RESET(STR)\
    do{\
        if(STR && STR->_str){\
            memset(STR->_str, 0, STR->_capacity);\
        }\
    }while(0);

#define STRINGER_PRINT(STR)\
        if(STR && STR->_str)\
            printf("%s", STR->_str);\

#define STRINGER_PRINTLN(STR)\
        if(STR && STR->_str)\
            printf("%s\n", STR->_str);\

#define STRINGER_NULL(STR) (STR == NULL)

#define STRINGER_FREE(STR)\
        if(!STRINGER_NULL(STR->_str)){\
            free(STR->_str);\
            STR->_str = NULL;\
            free(STR);\
            STR = NULL;\
        }
#endif

