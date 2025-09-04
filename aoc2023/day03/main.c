#include <ctype.h>
#include <errno.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define return_defer(value)                                                    \
  do {                                                                         \
    result = (value);                                                          \
    goto defer;                                                                \
  } while (0)

typedef int Errno;

#define ARENA_IMPLEMENTATION
#include "arena.h"

static Arena default_arena = {0};
static Arena *context_arena = &default_arena;

static void *context_alloc(size_t size) {
  assert(context_arena);
  return arena_alloc(context_arena, size);
}

static void *context_realloc(void *oldp, size_t oldsz, size_t newsz) {
  if (newsz <= oldsz)
    return oldp;
  return memcpy(context_alloc(newsz), oldp, oldsz);
}

#define INPUT "./day03/test.txt"
// #define INPUT "./day03/input.txt"

static char *contents;

Errno read_entire_file(const char *file_path, char **buffer,
                       size_t *buffer_size) {
  Errno result = 0;
  FILE *f = NULL;

  f = fopen(file_path, "rb");
  if (f == NULL)
    return_defer(errno);

  if (fseek(f, 0, SEEK_END) < 0)
    return_defer(errno);
  long m = ftell(f);
  if (m < 0)
    return_defer(errno);
  if (fseek(f, 0, SEEK_SET) < 0)
    return_defer(errno);

  *buffer_size = m;
  *buffer = context_alloc(*buffer_size);

  fread(*buffer, *buffer_size, 1, f);
  if (ferror(f))
    return_defer(errno);

defer:
  if (f)
    fclose(f);
  return result;
}

int read_from_file(char *filename) {
  FILE *f = NULL;
  int result = 0;
  {
    f = fopen(filename, "r");
    if (f == NULL) {
      fprintf(stderr, "%s: ERROR: could not read the file: %s\n", filename,
              strerror(errno));
      return_defer(errno);
    }

    fseek(f, 0, SEEK_END);
    int length = ftell(f);
    fseek(f, 0, SEEK_SET);

    contents = malloc(sizeof(char) * length + 1);
    fread(contents, 1, length, f);
    contents[length] = '\0';
  }
defer:
  fclose(f);
  return result;
}

int check_for_number(char *buffer, size_t buffer_size, int rowlen, int x,
                    int collen, int y) {

  for (int i = -1; i < 2; ++i) {
    int dy = y + i;
    if (dy >= 0 && dy < collen) {
      for (int ii = -1; ii < 2; ++ii) {
        int dx = x + ii;
        if (dx > 0 && dx < rowlen) {
          int c = (dx - 1) + (dy * rowlen);
          if (isdigit(buffer[c])) {
            printf("c: %d \n", c);
          }
        }
      }
    }
  }
  return 1;
}

bool check_for_sign(char *buffer, size_t buffer_size, int rowlen, int x,
                    int collen, int y) {

  for (int i = -1; i < 2; ++i) {
    int dy = y + i;
    if (dy >= 0 && dy < collen) {
      for (int ii = -1; ii < 2; ++ii) {
        int dx = x + ii;
        if (dx > 0 && dx < rowlen) {
          int c = (dx - 1) + (dy * rowlen);
          if (!isdigit(buffer[c]) && buffer[c] != '.') {
            return true;
          }
        }
      }
    }
  }
  return false;
}

int main(void) {
  int result = 0;

  // result = read_from_file(INPUT);

  char *buffer;
  size_t buffer_size;
  Errno err = read_entire_file(INPUT, &buffer, &buffer_size);
  if (err != 0) {
    fprintf(stderr, "ERROR: could not read file %s: %s\n", INPUT,
            strerror(errno));
    return_defer(1);
  }

  printf("%s\n", buffer);
  contents = buffer;
  int rowlen = 0;
  long sum = 0;
  bool sign = false;
  char *tmp;
  tmp = context_alloc(sizeof(char) * 8);
  tmp[0] = '\0';
  for (int x = 0; contents[x] != '\0'; x++) {
    if (contents[x] == '\n') {
      rowlen = x + 1;
      break;
    }
  }
  printf("------------------------------------\n");
  printf(" rowlen = %d\n", rowlen);
  printf(" buffer size = %zu\n", buffer_size);
  printf(" rowlen = %f\n", buffer_size / (float)rowlen);
  printf("------------------------------------\n");

  int c = 0;
  int collen = (int)(buffer_size / (float)rowlen);
  /*
  for (int y = 0; y < collen; ++y) {
    for (int x = 1; x < rowlen; ++x) {
      c = (x - 1) + (y * rowlen);
      // printf("%c", contents[c]);
      if (isdigit(contents[c])) {
        int tmplen = strlen(tmp);
        tmp[tmplen] = contents[c];
        tmp[tmplen + 1] = '\0';
        if (!sign)
          sign = check_for_sign(buffer, buffer_size, rowlen, x, collen, y);
      } else if (strlen(tmp) > 0) {
        if (sign)
          printf("tmp: %s\n", tmp);
        if (sign)
          sum += atol(tmp);
        sign = false;
        tmp[0] = '\0';
      }
    }
    if (strlen(tmp) > 0) {
      if (sign)
        printf("tmp: %s\n", tmp);
      if (sign)
        sum += atol(tmp);
      sign = false;
      tmp[0] = '\0';
    }
    printf("\n");
  }
  printf("Sum %ld\n\n", sum);
*/
  for (int y = 0; y < collen; ++y) {
    for (int x = 1; x < rowlen; ++x) {
      c = (x - 1) + (y * rowlen);
      // printf("%c", contents[c]);
      if (contents[c] == '*') {
        printf("x: %d  y: %d\n", x, y);
        sum += check_for_number(buffer, buffer_size, rowlen, x, collen, y);
      }
    }
    // printf("\n");
  }

  printf("Sum %ld\n\n", sum);


defer:
  return result;
}
