#pragma once

#ifdef ARDUINO
#include "Arduino.h"
#else
#include <cmath>
#endif

template<typename T>
struct Triple {
  T x;
  T y;
  T z;

  constexpr Triple<T>() : x{}, y{}, z{} {};
  constexpr Triple<T>(T all_) : x{all_}, y{all_}, z{all_} {};
  constexpr Triple<T>(T x_, T y_, T z_) : x{x_}, y{y_}, z{z_} {};

  Triple<T>& operator=(const Triple<T>& o) = default;
  Triple<T>& operator=(T all_) {
    return *this = Triple<T>{all_};
  }

  constexpr Triple<T> operator+() const {
    return *this;
  }

  constexpr Triple<T> operator-() const {
    return Triple<T>{-x, -y, -z};
  }

  constexpr Triple<T> operator+(const Triple<T>& o) const {
    return Triple<T>{x + o.x, y + o.y, z + o.z};
  }

  constexpr Triple<T> operator-(const Triple<T>& o) const {
    return operator+(-o);
  }

  template<typename S>
  constexpr auto operator/(S o) -> Triple<decltype(x / o)> const {
    return Triple<decltype(x / o)>{x / o, y / o, z / o};
  }

  template<typename S>
  constexpr auto operator*(S o) -> Triple<decltype(x * o)> const {
    return Triple<decltype(x * o)>{x * o, y * o, z * o};
  }

  Triple<T> operator+=(const Triple<T>& o) {
    return *this = *this + o;
  }

  Triple<T> operator-=(const Triple<T>& o) {
    return *this = *this - o;
  }

  template<typename S>
  Triple<T> operator/=(S o) {
    return *this = *this / o;
  }

  template<typename S>
  Triple<T> operator*=(S o) {
    return *this = *this * o;
  }

  constexpr T length() const {
    return sqrt(x*x + y*y + z*z);
  }
};

#ifdef ARDUINO
template<typename T>
void printTriple(const Triple<T> t) {
  if (t.x >= 0) Serial.print("+");
  Serial.print(t.x, 3);
  Serial.print(" ");
  if (t.y >= 0) Serial.print("+");
  Serial.print(t.y, 3);
  Serial.print(" ");
  if (t.z >= 0) Serial.print("+");
  Serial.print(t.z, 3);
}
#endif
