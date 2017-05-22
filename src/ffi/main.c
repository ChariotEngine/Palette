#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>

extern void palette_free(char *ptr_colors_arr
                        ,size_t len);

extern ssize_t palette_new_from_file(const char *file_path
                                    ,char **colors_arr
                                    ,size_t *len);

const int ERR_NO_ARG = 1;

int main(int argc, char **argv) {
  if (argc != 2) {
    printf("usage: cpalette <path/to/your.bin>\n");
    return ERR_NO_ARG;
  }

  char *file_path = argv[1];
  char *ptr_colors_arr = NULL;
  size_t len = 0;

  ssize_t code = palette_new_from_file(file_path, &ptr_colors_arr, &len);
  if (code != 0) {
    switch (code) {
      case 1: printf("'file_path' was null!"); break;
      case 2: printf("'file_path' contained non-utf8 characters!"); break;
      case -1: printf("Invalid palette!"); break;
      case -32767: printf("An unknown error occurred while reading the palette."); break;
    }

    return code;
  }

  printf("palette len: %zu\n", len);
  palette_free(ptr_colors_arr, len);
}
