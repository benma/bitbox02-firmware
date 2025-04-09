// Copyright 2025 Shift Crypto AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef _MEMORY_SPI_H_
#define _MEMORY_SPI_H_

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include <compiler_util.h>

#define MEMORY_SPI_ERASE_GRANULARITY 4096

// BLE firmware max size is 32kB.
#define MEMORY_SPI_BLE_FIRMWARE_MAX_SIZE (32 * 1024)
// The first 64kB are reserved for storing BLE firmwares, so we can safely upgrade.
#define MEMORY_SPI_BLE_FIRMWARE_1_ADDR 0x00
#define MEMORY_SPI_BLE_FIRMWARE_2_ADDR MEMORY_SPI_BLE_FIRMWARE_MAX_SIZE

#if (MEMORY_SPI_BLE_FIRMWARE_1_ADDR % MEMORY_SPI_ERASE_GRANULARITY)
#error "Address must be aligned to an erase sector"
#endif
#if (MEMORY_SPI_BLE_FIRMWARE_2_ADDR % MEMORY_SPI_ERASE_GRANULARITY)
#error "Address must be aligned to an erase sector"
#endif

USE_RESULT bool memory_spi_get_active_ble_firmware(
    uint8_t** firmware_out,
    size_t* size_out,
    uint8_t* checksum_out);

#endif
