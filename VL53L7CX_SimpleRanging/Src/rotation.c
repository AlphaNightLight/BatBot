#include "rotation.h"
#include "math.h"

Vector rotate(const Vector* v, const Vector* a) {
  // cx around axis z, cy around axis y, cz around axis x
  double cx = cos(a->z), cy = cos(a->y), cz = cos(a->x);
  double sx = sin(a->z), sy = sin(a->y), sz = sin(a->x);

  Vector result;
  result.x = cx*cy*v->x + (cx*sy*sz-sx*cz)*v->y + (cx*sy*cz+sx*sz)*v->z;
  result.y = sx*cy*v->x + (sx*sy*sz+cx*cz)*v->y + (sx*sy*cz-cx*sz)*v->z;
  result.z =   -sy*v->x +     cy*sz       *v->y +     cy*cz       *v->z;

  return result;
}
