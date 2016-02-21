// compile with `clang -lostn02_phf -L target/release -o ostn02_shifts  src/ostn02.c` from project root
// run with `LD_LIBRARY_PATH=target/release ./ostn02_shifts` from project root

#include <stdio.h>
#include <stdint.h>

typedef struct {
  uint32_t easting;
  uint32_t northing;
} gridrefs;

typedef struct {
  double x_shift;
  double y_shift;
  double z_shift;
} adjustment;

extern adjustment get_shifts_ffi(gridrefs);

int main(void) {
  gridrefs initial = { .easting = 651, .northing = 313 };
  adjustment adj = get_shifts_ffi(initial);
  // printf("(%d, %d, %d)\n", adj.x_shift, adj.y_shift, adj.z_shift);
  printf("(%d, %d)\n", initial.easting, initial.northing);
  return 0;
}
