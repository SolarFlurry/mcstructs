#pragma once

#include <stdint.h>
#include <stddef.h>

#define toStringSlice(a, b) (StringSlice) {.len = b, .ptr = a}

typedef struct McStructure {
    void* inner;
} McStructure;

typedef struct Vec3 {
    uint32_t x, y, z;
} Vec3;

typedef struct StringSlice {
    uint8_t* ptr;
    size_t len;
} StringSlice;

typedef struct BlockType {
    void* inner;
} BlockType;

Vec3 vec3(uint32_t x, uint32_t y, uint32_t z);
McStructure mcstructure_new(Vec3 size);
void mcstructure_free(McStructure* structure);
size_t mcstructure_as_bytes(McStructure* structure, uint8_t** ptr, size_t* capacity);
void mcstructure_free_bytes(uint8_t* ptr, size_t capacity, size_t len);
void mcstructure_setblock(McStructure* structure, Vec3 loc, BlockType block);
BlockType blocktype_new(StringSlice ns);
void blocktype_free(BlockType* block);