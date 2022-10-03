/* * keyvi - A key value store.
 *
 * Copyright 2015, 2016 Narek Gharibyan <narekgharibyan@gmail.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

/*
 * c_api.h
 *
 *  Created on: September 4, 2017
 *      Author: Narek Gharibyan <narekgharibyan@gmail.com>
 */

#ifndef KEYVI_C_API_C_API_H_
#define KEYVI_C_API_C_API_H_

#ifdef __cplusplus
extern "C" {
#endif

#include <stddef.h>
#include <stdint.h>

struct keyvi_dictionary;
struct keyvi_match;
struct keyvi_match_iterator;
struct keyvi_json_dictionary_compiler;
struct keyvi_json_dictionary_merger;
struct keyvi_completion_dictionary_compiler;
struct keyvi_completion_dictionary_merger;

struct keyvi_bytes {
  const size_t data_size;
  const uint8_t* const data_ptr;
};

//////////////////////
//// Bytes
//////////////////////

void keyvi_bytes_destroy(keyvi_bytes bytes);

//////////////////////
//// String
//////////////////////

void keyvi_string_destroy(char* str);

//////////////////////
//// Dictionary
//////////////////////

struct keyvi_dictionary* keyvi_create_dictionary(const char*);

void keyvi_dictionary_destroy(const struct keyvi_dictionary*);

size_t keyvi_dictionary_get_size(const struct keyvi_dictionary*);

char* keyvi_dictionary_get_statistics(const struct keyvi_dictionary*);

struct keyvi_match* keyvi_dictionary_get(const struct keyvi_dictionary*, const char*, const size_t);

struct keyvi_match_iterator* keyvi_dictionary_get_all_items(const struct keyvi_dictionary*);

struct keyvi_match_iterator* keyvi_dictionary_get_prefix_completions(const struct keyvi_dictionary*, const char*,
                                                                     const size_t, const size_t);

struct keyvi_match_iterator* keyvi_dictionary_get_fuzzy(const struct keyvi_dictionary*, const char*, const size_t,
                                                        const size_t);

struct keyvi_match_iterator* keyvi_dictionary_get_multi_word_completions(const struct keyvi_dictionary*, const char*,
                                                                         const size_t, const size_t);

//////////////////////
//// Match
//////////////////////

void keyvi_match_destroy(const struct keyvi_match*);

bool keyvi_match_is_empty(const struct keyvi_match*);

double keyvi_match_get_score(const struct keyvi_match*);

char* keyvi_match_get_value_as_string(const struct keyvi_match*);

keyvi_bytes keyvi_match_get_msgpacked_value(const struct keyvi_match*);

char* keyvi_match_get_matched_string(const struct keyvi_match*);

//////////////////////
//// Match Iterator
//////////////////////

void keyvi_match_iterator_destroy(const struct keyvi_match_iterator*);

bool keyvi_match_iterator_empty(const struct keyvi_match_iterator*);

struct keyvi_match* keyvi_match_iterator_dereference(const struct keyvi_match_iterator*);

void keyvi_match_iterator_increment(struct keyvi_match_iterator*);

/////////////////////////////
//// Json Dictionary Compiler
/////////////////////////////

struct keyvi_json_dictionary_compiler* keyvi_create_json_dictionary_compiler();

void keyvi_json_dictionary_compiler_add(struct keyvi_json_dictionary_compiler*, const char*, const size_t, const char*, const size_t);

void keyvi_json_dictionary_compiler_compile(struct keyvi_json_dictionary_compiler*);

void keyvi_json_dictionary_compiler_write_to_file(struct keyvi_json_dictionary_compiler*, const char*, const size_t);

void keyvi_json_dictionary_compiler_destroy(struct keyvi_json_dictionary_compiler*);

/////////////////////////////
//// Json Dictionary Merger
/////////////////////////////

struct keyvi_json_dictionary_merger* keyvi_create_json_dictionary_merger();

void keyvi_json_dictionary_merger_add(struct keyvi_json_dictionary_merger*, const char*, const size_t);

void keyvi_json_dictionary_merger_merge(struct keyvi_json_dictionary_merger*, const char*, const size_t);

void keyvi_json_dictionary_merger_destroy(struct keyvi_json_dictionary_merger*);

/////////////////////////////
//// Completion Dictionary Merger
/////////////////////////////

struct keyvi_completion_dictionary_merger* keyvi_create_completion_dictionary_merger();

void keyvi_completion_dictionary_merger_add(struct keyvi_completion_dictionary_merger*, const char*, const size_t);

void keyvi_completion_dictionary_merger_merge(struct keyvi_completion_dictionary_merger*, const char*, const size_t);

void keyvi_completion_dictionary_merger_destroy(struct keyvi_completion_dictionary_merger*);

/////////////////////////////
//// Completion Dictionary Compiler
/////////////////////////////

struct keyvi_completion_dictionary_compiler* keyvi_create_completion_dictionary_compiler();

void keyvi_completion_dictionary_compiler_add(struct keyvi_completion_dictionary_compiler*, const char*, const size_t, const size_t);

void keyvi_completion_dictionary_compiler_compile(struct keyvi_completion_dictionary_compiler*);

void keyvi_completion_dictionary_compiler_write_to_file(struct keyvi_completion_dictionary_compiler*, const char*, const size_t);

void keyvi_completion_dictionary_compiler_destroy(struct keyvi_completion_dictionary_compiler*);
#ifdef __cplusplus
} /* end extern "C" */
#endif

#endif  // KEYVI_C_API_C_API_H_
