#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define return_defer(value)                                                    \
  do {                                                                         \
    result = (value);                                                          \
    goto defer;                                                                \
  } while (0)

#define INPUT "./day03/test.txt"

static char *contents;

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

int main(void) {
  int result;

  result = read_from_file(INPUT);

  printf("%s\n", contents);
  return result;
}
