// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use anonima_bitfield::BitField;

/// An example bit field.
pub fn example1() -> BitField {
    BitField::from_bytes(EXAMPLE1).unwrap()
}

/// Another example bit field.
pub fn example2() -> BitField {
    BitField::from_bytes(EXAMPLE2).unwrap()
}

// taken from go-bitfield
// properties:
// - 1362 runs
//   - 54 single bit
//   - 399 small block
//   - 909 large block
// - 9955 set bits between 0 and ~1M
// - 681 ranges
const EXAMPLE1: &[u8] = &[
    0x20, 0xfc, 0x40, 0xc2, 0xcc, 0xe5, 0xd8, 0xc1, 0xe1, 0x1e, 0x23, 0xd3, 0x02, 0x2e, 0xcd, 0x03,
    0x3e, 0x83, 0x16, 0x26, 0x3a, 0x5c, 0x30, 0x8e, 0x00, 0x05, 0xcc, 0x24, 0x0e, 0x96, 0x15, 0x48,
    0xa0, 0x2a, 0x40, 0x04, 0x92, 0x0e, 0x94, 0xc0, 0x48, 0x91, 0xee, 0x05, 0x28, 0x98, 0x55, 0x90,
    0xa0, 0xa4, 0x50, 0x98, 0x14, 0x40, 0x04, 0x59, 0x0a, 0x22, 0x00, 0x74, 0xb0, 0x40, 0x66, 0x30,
    0xf9, 0x66, 0x90, 0x51, 0xc7, 0x70, 0x74, 0x40, 0x48, 0x7f, 0xf0, 0x80, 0x24, 0x20, 0x85, 0x58,
    0x06, 0x07, 0x66, 0x04, 0x87, 0xe5, 0x05, 0x28, 0x00, 0xa4, 0xf0, 0x61, 0x2e, 0x90, 0x08, 0x44,
    0x70, 0x38, 0x34, 0xf0, 0x08, 0x4f, 0x70, 0x68, 0xad, 0xd0, 0x90, 0x1e, 0x90, 0x38, 0xc1, 0x85,
    0x76, 0x04, 0x15, 0x7c, 0x04, 0x28, 0x28, 0x17, 0x70, 0xe0, 0x15, 0x00, 0x82, 0xfb, 0x11, 0x0b,
    0x76, 0x09, 0x22, 0xb8, 0x2f, 0x90, 0x20, 0x5f, 0x80, 0x84, 0xc9, 0x10, 0x85, 0x66, 0x09, 0x05,
    0xc9, 0x03, 0x2d, 0x19, 0xa4, 0x5a, 0x70, 0x45, 0xa5, 0x40, 0xc2, 0x05, 0x6c, 0x4a, 0x0f, 0x64,
    0xf4, 0x19, 0xb0, 0x40, 0x28, 0x8b, 0x02, 0x6c, 0x20, 0x3e, 0x90, 0x40, 0x01, 0x19, 0xea, 0x09,
    0x20, 0x60, 0x2f, 0x50, 0x60, 0x15, 0x00, 0x04, 0x69, 0x06, 0xb1, 0x8c, 0xa9, 0x85, 0xc6, 0x1f,
    0x13, 0x54, 0x3e, 0x58, 0x40, 0x17, 0x60, 0x41, 0x2e, 0x8a, 0x42, 0x3c, 0x8b, 0x0b, 0x3f, 0x08,
    0x10, 0x5c, 0x32, 0x38, 0xd4, 0x1e, 0x68, 0x18, 0x5b, 0x70, 0xc1, 0x2c, 0x06, 0x17, 0x6c, 0x17,
    0xf8, 0x74, 0x36, 0x28, 0x9c, 0x0e, 0x20, 0x02, 0xa1, 0x84, 0x0f, 0xa2, 0x82, 0x0e, 0xbf, 0x82,
    0x45, 0x29, 0x84, 0x07, 0x16, 0x25, 0x10, 0x12, 0x38, 0x8c, 0x13, 0x50, 0x41, 0xff, 0x03, 0x13,
    0x70, 0x16, 0x80, 0x81, 0x63, 0x85, 0x8f, 0xa0, 0x81, 0x0a, 0xe1, 0x80, 0x84, 0xa3, 0x81, 0x89,
    0xff, 0x84, 0xc7, 0x3c, 0x87, 0x49, 0xbb, 0x81, 0x4b, 0x21, 0x82, 0x03, 0x2f, 0x83, 0x84, 0x29,
    0x83, 0x8c, 0xe0, 0x00, 0x40, 0x8c, 0xda, 0x20, 0x42, 0x06, 0x13, 0x64, 0x07, 0x24, 0x85, 0x0c,
    0xa6, 0x21, 0x24, 0x6c, 0x07, 0x74, 0x20, 0x13, 0x3c, 0x02, 0x13, 0x54, 0xba, 0x1d, 0x14, 0x9e,
    0x0b, 0xe0, 0x00, 0x39, 0x82, 0x0a, 0x72, 0x0b, 0x80, 0x60, 0x53, 0x20, 0xd5, 0x0e, 0x32, 0xd5,
    0x0e, 0x12, 0xb9, 0x01, 0x44, 0x90, 0x27, 0x51, 0x59, 0xe0, 0x12, 0xc3, 0x04, 0xe8, 0x01, 0x78,
    0x40, 0xd8, 0xe0, 0x03, 0x7b, 0x60, 0x93, 0x6b, 0x20, 0x72, 0xf8, 0x20, 0xf1, 0xe1, 0xe0, 0x4d,
    0x90, 0xdf, 0x02, 0x1f, 0x69, 0x03, 0x36, 0x30, 0x8d, 0x20, 0x82, 0x58, 0x0a, 0x8f, 0xec, 0x02,
    0x8d, 0xe8, 0x00, 0x11, 0x55, 0x81, 0x58, 0x17, 0xa0, 0x82, 0xa5, 0x81, 0x88, 0xb6, 0x83, 0x88,
    0xed, 0x86, 0x88, 0xe0, 0x86, 0x4b, 0x6e, 0x82, 0x45, 0xe1, 0x89, 0xc8, 0x2c, 0x41, 0x78, 0x09,
    0x12, 0xbd, 0x91, 0x50, 0x01, 0x42, 0x90, 0x85, 0x70, 0x69, 0xb5, 0x20, 0x83, 0x5e, 0x02, 0x8d,
    0xfb, 0x02, 0x0f, 0xfb, 0x0a, 0x09, 0xcc, 0x04, 0x8b, 0xe4, 0x05, 0x87, 0xc9, 0x02, 0x0f, 0xfc,
    0x01, 0x5c, 0x08, 0x0f, 0xd0, 0x01, 0x4e, 0xd0, 0x60, 0x4d, 0x50, 0xa1, 0x3c, 0xf0, 0x20, 0x6c,
    0x88, 0xe1, 0x41, 0x5c, 0x11, 0x1c, 0xc8, 0x25, 0x4c, 0xc0, 0x1d, 0x1c, 0x4e, 0x61, 0x54, 0x2a,
    0x05, 0x14, 0xf6, 0x54, 0x3c, 0x0b, 0x44, 0xe4, 0x05, 0x64, 0x42, 0x1f, 0x64, 0x32, 0x23, 0xa0,
    0x20, 0xf9, 0x01, 0x0c, 0x40, 0x27, 0xf8, 0x40, 0x3c, 0xc0, 0x82, 0x72, 0x01, 0x0d, 0xd1, 0x43,
    0x11, 0x01, 0x0d, 0xaa, 0x35, 0xc8, 0xe0, 0x1e, 0x42, 0x41, 0x5b, 0x20, 0x8b, 0x04, 0x0e, 0xc9,
    0x03, 0x0a, 0xea, 0x01, 0x3e, 0x5f, 0x12, 0xab, 0x07, 0x40, 0xf0, 0xad, 0x60, 0x42, 0x6f, 0xa1,
    0xd2, 0x39, 0x60, 0x91, 0x64, 0xf1, 0xe4, 0x10, 0x3f, 0x60, 0xc2, 0x5c, 0x94, 0x62, 0x07, 0x1f,
    0x67, 0x05, 0x87, 0xcc, 0x07, 0x9f, 0xee, 0x0e, 0x13, 0xd8, 0x02, 0x38, 0xa0, 0xb5, 0x20, 0x02,
    0x4e, 0x01, 0x85, 0x7c, 0x0c, 0x4e, 0xb0, 0xde, 0x90, 0x80, 0x2c, 0x50, 0xd8, 0x77, 0x88, 0xf6,
    0x80, 0x04, 0x22, 0x85, 0xce, 0xf2, 0x82, 0xcb, 0xb4, 0x05, 0x14, 0x68, 0xa9, 0x70, 0x77, 0xb8,
    0x34, 0x1b, 0xf0, 0x01, 0xec, 0x01, 0x13, 0x58, 0x1a, 0x78, 0x88, 0x0b, 0x58, 0x80, 0x27, 0x98,
    0x20, 0x4b, 0x50, 0x41, 0x69, 0x84, 0xc9, 0x63, 0x81, 0x08, 0x7b, 0x06, 0x10, 0xd8, 0x1e, 0x38,
    0x1c, 0x17, 0x68, 0x00, 0x2e, 0xa8, 0xe4, 0x23, 0x78, 0xdc, 0x0a, 0x28, 0x38, 0x0b, 0x28, 0xcc,
    0x1f, 0x40, 0x01, 0x6f, 0x83, 0x88, 0x75, 0x82, 0xc7, 0x8c, 0x04, 0xec, 0x08, 0x28, 0xcc, 0x7b,
    0xe4, 0xb8, 0x00, 0x0e, 0x46, 0x35, 0x2c, 0xc2, 0x2d, 0xb8, 0xe0, 0xb5, 0x42, 0xa2, 0x75, 0x40,
    0x02, 0xfc, 0xc4, 0x43, 0x98, 0x41, 0xa1, 0x56, 0x40, 0xc4, 0x5a, 0x01, 0x13, 0xda, 0x33, 0x5c,
    0xf2, 0x07, 0x44, 0x74, 0x3f, 0x5c, 0x5a, 0x49, 0x24, 0x64, 0x03, 0x2c, 0xd8, 0x23, 0x34, 0x78,
    0x19, 0xb8, 0x61, 0x15, 0x42, 0xa3, 0xc6, 0x46, 0x73, 0x80, 0x0a, 0x52, 0x17, 0x00, 0xa1, 0x7d,
    0x40, 0x26, 0x19, 0xc1, 0x06, 0xf0, 0x40, 0x45, 0x90, 0xc2, 0x81, 0xf6, 0xc0, 0x22, 0xf3, 0x80,
    0x15, 0x02, 0x07, 0x2c, 0x9a, 0x03, 0xd8, 0xc0, 0x95, 0x21, 0xbb, 0x04, 0x80, 0x00, 0x99, 0xe0,
    0x90, 0x19, 0x21, 0x93, 0xfe, 0x21, 0x43, 0x1a, 0x90, 0x64, 0x02, 0x2e, 0x20, 0x65, 0xc8, 0x79,
    0x86, 0xc4, 0x78, 0x04, 0x2a, 0x38, 0x9e, 0x50, 0x82, 0xaf, 0x05, 0x17, 0x50, 0x33, 0xa0, 0x42,
    0x21, 0x05, 0x24, 0xc0, 0x0f, 0x64, 0xb0, 0x40, 0xe5, 0x3a, 0x42, 0xc1, 0xfd, 0x02, 0x3c, 0x10,
    0x09, 0x88, 0x40, 0x5a, 0x40, 0x84, 0x11, 0x41, 0x83, 0x7f, 0x42, 0x82, 0x10, 0x03, 0x08, 0x8a,
    0x1b, 0xc8, 0xc0, 0xb2, 0x81, 0x16, 0x6e, 0x68, 0x21, 0x1a, 0x02, 0x11, 0x30, 0x00, 0x42, 0x71,
    0xc1, 0xe7, 0x89, 0x18, 0x60, 0x05, 0x3c, 0x70, 0x3c, 0xa2, 0x35, 0x14, 0xe6, 0x25, 0x98, 0x20,
    0x97, 0x20, 0xa8, 0x02, 0x26, 0x8e, 0x12, 0x6c, 0x30, 0x2a, 0xa0, 0x22, 0x5e, 0xc2, 0x05, 0x8f,
    0x08, 0x0a, 0xea, 0x04, 0x0a, 0x9b, 0x04, 0x3e, 0xf4, 0x03, 0x2e, 0x92, 0x05, 0x26, 0x65, 0xe0,
    0x28, 0x0e, 0x38, 0x50, 0xb0, 0x2f, 0x61, 0xe1, 0x6a, 0xa0, 0xf3, 0x2c, 0x12, 0x49, 0x06, 0x26,
    0x20, 0x84, 0x30, 0x51, 0x5d, 0x90, 0xb0, 0x57, 0x70, 0x11, 0x3d, 0x70, 0x10, 0x53, 0x1c, 0x98,
    0xf8, 0x54, 0x00, 0x02, 0xc3, 0x10, 0x58, 0xb0, 0x4d, 0xb0, 0x20, 0x0d, 0x71, 0x40, 0x4d, 0x20,
    0x04, 0x79, 0x05, 0x1f, 0x7b, 0x01, 0x89, 0x69, 0x02, 0x28, 0xc8, 0x17, 0x30, 0x81, 0x15, 0xd0,
    0xc0, 0x5f, 0x50, 0x19, 0x1e, 0x72, 0x4d, 0x8d, 0xc1, 0x01, 0x91, 0x7e, 0x05, 0x34, 0x30, 0x0e,
    0x91, 0xc9, 0x74, 0x20, 0x84, 0xd5, 0x13, 0x0b, 0x61, 0x0d, 0x8f, 0xd8, 0x00, 0x32, 0x88, 0x4e,
    0x20, 0x04, 0xd7, 0x84, 0xbc, 0x06, 0x38, 0x6c, 0x1f, 0xc8, 0xe0, 0x12, 0x68, 0xb4, 0x16, 0xa0,
    0x41, 0x3a, 0x07, 0x27, 0x90, 0x1a, 0x88, 0x18, 0x0a, 0xa5, 0x71, 0x25, 0xf1, 0x09, 0xa8, 0x50,
    0x9d, 0xe1, 0x82, 0xbc, 0x81, 0x04, 0xc9, 0x09, 0x7c, 0x10, 0x3e, 0x40, 0x05, 0xe9, 0x02, 0x3a,
    0xce, 0x0d, 0x5c, 0x70, 0xfa, 0x60, 0x91, 0x41, 0x09, 0xc7, 0x08, 0x0a, 0x93, 0x03, 0x7c, 0xa0,
    0x3f, 0xe0, 0x53, 0x4b, 0x40, 0x06, 0xea, 0x0d, 0x0a, 0xcd, 0x18, 0x0a, 0x83, 0x05, 0x2a, 0x29,
    0x0e, 0xc3, 0x01, 0x54, 0xb0, 0x7c, 0x41, 0x04, 0xaf, 0x02, 0x54, 0x40, 0xbe, 0x20, 0x41, 0xa7,
    0x21, 0x29, 0xe1, 0x62, 0x6a, 0x81, 0x08, 0xb8, 0x04, 0x64, 0xb0, 0x29, 0xa0, 0xb0, 0x7f, 0xe0,
    0x00, 0x3a, 0xc0, 0x05, 0xcb, 0x02, 0xb8, 0xe0, 0x6e, 0xc0, 0x08, 0xda, 0x13, 0x78, 0xe0, 0x1a,
    0x00, 0x04, 0xe2, 0x08, 0x0a, 0x1b, 0x16, 0x9d, 0x35, 0x88, 0x00, 0x0a, 0x61, 0xe3, 0x1d, 0x61,
    0xa1, 0x8f, 0xe0, 0xe3, 0x7d, 0x00, 0x08, 0x56, 0x16, 0xb9, 0x05, 0x7c, 0x50, 0x5a, 0xe2, 0xd1,
    0xc9, 0x40, 0x08, 0x93, 0x0a, 0x36, 0xc4, 0x02, 0x0e, 0xf7, 0x15, 0x0a, 0xae, 0x1b, 0x0a, 0x83,
    0x0d, 0x36, 0xe8, 0x0a, 0x16, 0xde, 0x11, 0x3e, 0x89, 0x0b, 0x50, 0x70, 0xd9, 0x10, 0xca, 0x00,
    0x38, 0xc0, 0x8f, 0xc1, 0x83, 0x77, 0x86, 0x0c, 0x1b, 0xe8, 0x00, 0x9e, 0x78, 0x08, 0x0b, 0xa0,
    0xc3, 0xb7, 0x84, 0x82, 0x18, 0x4b, 0x54, 0x3b, 0x70, 0x81, 0xbe, 0x41, 0xcc, 0x0d, 0x08, 0x61,
    0x8e, 0x17, 0x00, 0x0d, 0xd8, 0x40, 0x34, 0x20, 0xbd, 0x12, 0x0e, 0x83, 0x16, 0x3a, 0x8a, 0x16,
    0x48, 0xd0, 0x59, 0xe0, 0x21, 0x4c, 0x60, 0x42, 0xaa, 0xa0, 0x60, 0x69, 0x00, 0x05, 0xb9, 0x02,
    0x12, 0x94, 0x0f, 0x48, 0x60, 0xfe, 0x90, 0x66, 0x03, 0x4e, 0xf8, 0x4c, 0x50, 0xd8, 0x5f, 0x70,
    0x99, 0x1f, 0xd0, 0xf0, 0x55, 0x88, 0x74, 0x81, 0x0d, 0x6b, 0x80, 0x84, 0xb9, 0x8d, 0xc7, 0x34,
    0x81, 0x4e, 0xa4, 0x8a, 0x82, 0xa1, 0x02, 0x15, 0x40, 0x07, 0x70, 0x81, 0x7f, 0x8d, 0x03, 0x61,
    0x02, 0x11, 0x1c, 0x1e, 0x78, 0xb8, 0x1a, 0x80, 0x02, 0xa3, 0x80, 0x02, 0x69, 0x80, 0xc7, 0xf3,
    0x01, 0x2c, 0xdc, 0x13, 0x90, 0x81, 0xad, 0x81, 0x88, 0xb8, 0x81, 0x82, 0x78, 0x84, 0x4e, 0xb6,
    0x83, 0x45, 0xb3, 0x06, 0x37, 0x90, 0x5e, 0x68, 0x48, 0x82, 0x28, 0x00, 0xc1, 0x01, 0xe5, 0x80,
    0x86, 0xf2, 0x81, 0xcc, 0xb9, 0x81, 0x88, 0xeb, 0x80, 0x03, 0xb7, 0x87, 0x8d, 0x69, 0x83, 0x45,
    0x74, 0x05, 0x22, 0x84, 0x43, 0x48, 0x48, 0x9f, 0xa4, 0xf3, 0x40, 0xa4, 0x78, 0x80, 0x0b, 0xec,
    0x0b, 0x2c, 0x08, 0x07, 0x74, 0x88, 0x15, 0x34, 0xc2, 0xa2, 0xa1, 0x33, 0x0d, 0xc1, 0x08, 0x91,
    0x18, 0xf0, 0x60, 0x38, 0x40, 0x07, 0x83, 0x01, 0x8c, 0x10, 0x19, 0x40, 0x04, 0xbb, 0x1a, 0x68,
    0x30, 0x2b, 0x60, 0x92, 0xaa, 0x80, 0x08, 0xef, 0x02, 0x54, 0x50, 0x7f, 0x80, 0x0a, 0x89, 0x02,
    0x32, 0x94, 0x17, 0x1a, 0xe1, 0x04, 0x0e, 0xa6, 0x01, 0x0a, 0x97, 0x0b, 0x16, 0xe4, 0x0a, 0x91,
    0x3c, 0x90, 0xf9, 0x1e, 0x10, 0xf1, 0x25, 0x70, 0x29, 0x1d, 0xa0, 0x82, 0xf0, 0x04, 0x40, 0xe8,
    0x56, 0x40, 0x05, 0x54, 0x0f, 0x1b, 0xc6, 0x01, 0x2a, 0x98, 0x35, 0xf0, 0x30, 0xbd, 0x80, 0x07,
    0xd9, 0x06, 0x20, 0x90, 0x5e, 0x09, 0xf9, 0x0b, 0x1c, 0xac, 0x22, 0x20, 0xc1, 0x96, 0x4f, 0x70,
    0x80, 0x4b, 0xae, 0x81, 0x47, 0xb9, 0x8d, 0x8d, 0x3d, 0x82, 0x45, 0x65, 0x80, 0xca, 0xb9, 0x05,
    0x18, 0x6c, 0x26, 0xa4, 0x53, 0x40, 0xc7, 0xb2, 0xc2, 0x06, 0xdf, 0x80, 0x0a, 0xa0, 0x29, 0x30,
    0x23, 0x32, 0x81, 0x08, 0x7c, 0x3c, 0x3e, 0x37, 0xb8, 0x20, 0xd7, 0x80, 0x0b, 0xfc, 0x25, 0x24,
    0xe8, 0x1d, 0x2c, 0x50, 0x0d, 0x3c, 0xe0, 0x88, 0x60, 0x3a, 0x83, 0x0c, 0xa6, 0x11, 0xb2, 0x5c,
    0xc1, 0x07, 0xf2, 0x01, 0x2a, 0xce, 0x01, 0x12, 0xd7, 0x64, 0x0e, 0xb2, 0x07, 0xa9, 0x6e, 0xd0,
    0x71, 0x1d, 0x70, 0x80, 0xe5, 0x08, 0x6d, 0x83, 0x84, 0x6a, 0x00, 0x19, 0x78, 0x0e, 0x48, 0xc4,
    0x07, 0x80, 0x81, 0xae, 0x80, 0x44, 0x3f, 0x01, 0x13, 0x4c, 0x1f, 0x58, 0xe0, 0x4a, 0x48, 0x8c,
    0x2e, 0x04, 0x78, 0x81, 0x16, 0x7a, 0x05, 0x90, 0xa0, 0xdd, 0xc3, 0xa7, 0xd8, 0xc2, 0x44, 0xfc,
    0x40, 0x02, 0xda, 0x00, 0x0c, 0x54, 0x11, 0xe0, 0xc0, 0xba, 0x80, 0x09, 0x7a, 0x09, 0xa0, 0x00,
    0x52, 0x40, 0x61, 0x51, 0x44, 0x67, 0x12, 0x03, 0x0c, 0x00, 0x0f, 0x70, 0xe1, 0x31, 0xc0, 0xe1,
    0xb6, 0xc2, 0xa1, 0xb9, 0xc1, 0x46, 0x50, 0x40, 0x01, 0x9b, 0x01, 0x1a, 0xc2, 0x11, 0x72, 0x4c,
    0xa1, 0x02, 0x5b, 0x20, 0xb1, 0x28, 0xc0, 0x04, 0xc7, 0x12, 0x40, 0x00, 0xee, 0x00, 0x08, 0xd1,
    0x2a, 0x32, 0xd0, 0x11, 0x81, 0x7f, 0xa0, 0x84, 0x77, 0x0a, 0x93, 0xc8, 0x00, 0x97, 0xe7, 0x01,
    0x09, 0x50, 0x01, 0x05, 0xf5, 0x00, 0x85, 0xee, 0x03, 0x93, 0x59, 0x82, 0x3c, 0x59, 0x14, 0x52,
    0x28, 0xa4, 0x0b, 0x28, 0x30, 0x0a, 0x84, 0x4d, 0x81, 0x7f, 0x43, 0xa7, 0x56, 0x80, 0x0b, 0x92,
    0x0f, 0x3c, 0x98, 0x47, 0xa8, 0x00, 0x35, 0x20, 0xde, 0x03, 0x2a, 0x8b, 0x02, 0x2e, 0x8f, 0x06,
    0x58, 0x40, 0x58, 0xe0, 0x33, 0x4d, 0xc1, 0x0a, 0xc0, 0x06, 0xc0, 0xc0, 0x04, 0x05, 0xf5, 0x04,
    0x16, 0xfa, 0x26, 0x60, 0x60, 0x9c, 0xa0, 0x63, 0x2f, 0x20, 0x13, 0xbe, 0x81, 0x0c, 0x97, 0x0d,
    0x32, 0xd0, 0x01, 0x22, 0x3e, 0x80, 0x20, 0x49, 0xe0, 0x50, 0x18, 0x21, 0x02, 0x19, 0xe0, 0x20,
    0xbd, 0x10, 0xc8, 0x04, 0x9b, 0x40, 0x0d, 0x24, 0xe0, 0xcd, 0x90, 0xe9, 0x03, 0x82, 0x16, 0x11,
    0x28, 0x2a, 0xd0, 0xb5, 0x00, 0x83, 0x73, 0x0b, 0x19, 0xd6, 0x07, 0x1b, 0x71, 0x05, 0x26, 0x28,
    0x6d, 0x00, 0x82, 0x7f, 0x05, 0x2a, 0xb8, 0x56, 0x80, 0x84, 0x79, 0x04, 0x86, 0xa0, 0x7e, 0x50,
    0x20, 0xe3, 0x03, 0x20, 0x36, 0x88, 0xaf, 0x90, 0x91, 0xd5, 0x60, 0x82, 0xe4, 0x01, 0x93, 0xf4,
    0x08, 0x95, 0xc0, 0x05, 0x1b, 0x61, 0x06, 0x87, 0xc3, 0x01, 0x47, 0xb5, 0x41, 0x51, 0xc9, 0x26,
    0x00, 0x03, 0xc1, 0x00, 0x87, 0x46, 0x05, 0x8f, 0x5a, 0x01, 0x95, 0xef, 0x09, 0x89, 0xca, 0x00,
    0x8b, 0xf7, 0x02, 0x19, 0x5b, 0x0b, 0x32, 0x08, 0x82, 0x02, 0xec, 0x00, 0x13, 0x23, 0x11, 0x40,
    0x04, 0x20, 0xe0, 0x26, 0x51, 0x89, 0x85, 0x40, 0x84, 0xc2, 0x01, 0x0f, 0x51, 0x01, 0x0d, 0x08,
    0x6e, 0xf8, 0x6e, 0x00, 0x82, 0x6a, 0x01, 0x0f, 0x72, 0x04, 0x2c, 0x40, 0x4d, 0x50, 0x41, 0x16,
    0x08, 0x67, 0x46, 0x4a, 0x0d, 0x90, 0xc0, 0xfe, 0xc1, 0x42, 0x30, 0x01, 0x09, 0x50, 0x0d, 0x5c,
    0x16, 0x09, 0xc0, 0x00, 0xf6, 0x41, 0x27, 0x92, 0xc0, 0x63, 0x7a, 0xc2, 0xa1, 0x9c, 0x40, 0x21,
    0x27, 0xec, 0x25, 0x4c, 0x80, 0x1b, 0x20, 0x81, 0x2b, 0xa0, 0x00, 0x18, 0xe0, 0xe1, 0x6a, 0x21,
    0x02, 0xa3, 0xd2, 0xaf, 0xe0, 0x03, 0x7a, 0x00, 0x04, 0x99, 0x26, 0x50, 0x70, 0xcf, 0xa0, 0xe1,
    0x58, 0xa1, 0xd3, 0x38, 0xa0, 0x30, 0x79, 0x20, 0xd2, 0x3a, 0x20, 0xb3, 0x5e, 0x60, 0xa1, 0xeb,
    0x80, 0x05, 0xc8, 0x01, 0x4c, 0xc0, 0x5b, 0xa0, 0x60, 0xc8, 0x41, 0x04, 0x6d, 0xc8, 0x20, 0xab,
    0x10, 0xe3, 0x07, 0x1f, 0x4b, 0x07, 0x28, 0x70, 0x17, 0x80, 0x84, 0xf9, 0x06, 0x46, 0x90, 0x44,
    0xf0, 0x40, 0xf6, 0x60, 0x05, 0xca, 0x07, 0x1b, 0xcf, 0x02, 0x0b, 0x61, 0x12, 0x15,
];

// the same runs as the first one but shuffled (the runs of 1s are still runs of 1s, just in a
// different location), so it has the exact same properties
const EXAMPLE2: &[u8] = &[
    0xa0, 0xb8, 0x81, 0x0e, 0x4a, 0x21, 0x5c, 0xaa, 0x05, 0x14, 0x60, 0x0f, 0x82, 0x1c, 0xa1, 0xc0,
    0x3c, 0x02, 0x06, 0xe9, 0x02, 0xdc, 0x90, 0x2f, 0xe0, 0x01, 0x21, 0x31, 0x3d, 0xe1, 0x60, 0x1c,
    0xe2, 0x63, 0x84, 0x0a, 0x93, 0x03, 0x1e, 0xe9, 0x1f, 0x12, 0x99, 0x0e, 0xf0, 0x70, 0xad, 0x80,
    0x04, 0xa9, 0x0a, 0x00, 0x91, 0x4d, 0xa0, 0xf3, 0x3d, 0xe1, 0xd1, 0x6a, 0xa1, 0xd3, 0x39, 0x80,
    0x8b, 0x68, 0x22, 0x2a, 0x90, 0x24, 0x05, 0xf6, 0x08, 0x8b, 0x54, 0x0b, 0x91, 0x3c, 0x9d, 0xe7,
    0x01, 0x0b, 0x52, 0x85, 0x4c, 0x2a, 0xf0, 0x82, 0xab, 0x01, 0x12, 0xa8, 0x63, 0xe4, 0x74, 0x20,
    0x29, 0x81, 0x3d, 0x40, 0x85, 0x5c, 0x02, 0x28, 0x90, 0x2c, 0x70, 0x98, 0xc5, 0x60, 0x82, 0x5a,
    0x01, 0x93, 0xcc, 0x07, 0x09, 0x77, 0x01, 0x15, 0x42, 0x0c, 0x8b, 0x49, 0x01, 0x2e, 0xe8, 0x56,
    0x20, 0x02, 0xde, 0x0c, 0x8d, 0xdc, 0x02, 0x87, 0x74, 0x0e, 0x07, 0x54, 0x0f, 0x28, 0x08, 0x44,
    0x30, 0xd9, 0x77, 0x50, 0x91, 0x3d, 0xe0, 0x02, 0xed, 0x09, 0x15, 0xd0, 0x01, 0x87, 0x99, 0x3c,
    0x8b, 0x10, 0x81, 0x74, 0x81, 0x87, 0xf8, 0x80, 0x08, 0x39, 0x82, 0xc6, 0xa0, 0x05, 0x21, 0xd4,
    0x13, 0x58, 0x44, 0x06, 0x58, 0xd4, 0x12, 0x88, 0x44, 0x57, 0x10, 0x01, 0x64, 0x82, 0x0e, 0x61,
    0x81, 0xc3, 0x38, 0x81, 0xc8, 0x2c, 0x81, 0x08, 0x1b, 0x14, 0x30, 0x23, 0x38, 0x8c, 0x6a, 0xd8,
    0x48, 0x9f, 0xf8, 0x5c, 0x36, 0x50, 0x82, 0x68, 0x00, 0x27, 0xb4, 0x0f, 0x80, 0x81, 0x78, 0x84,
    0xcb, 0xa2, 0x40, 0x58, 0x03, 0x24, 0x7a, 0x25, 0x4c, 0xc2, 0x13, 0x14, 0xe2, 0x59, 0x30, 0x41,
    0x58, 0x03, 0x08, 0xf6, 0x0f, 0x24, 0x6e, 0x24, 0x66, 0x09, 0x10, 0x01, 0x9a, 0xc2, 0x44, 0x36,
    0x40, 0x05, 0x57, 0x82, 0x12, 0x9e, 0x70, 0x21, 0x7a, 0xc3, 0x02, 0xf6, 0x81, 0x10, 0x90, 0x25,
    0xb8, 0x80, 0x57, 0xc0, 0xe3, 0xb6, 0x02, 0x19, 0xba, 0x0f, 0xa8, 0x80, 0xbd, 0xc0, 0x21, 0x9c,
    0x40, 0x41, 0xda, 0x00, 0x0e, 0x9c, 0x05, 0x7c, 0xe8, 0x07, 0x98, 0x80, 0x34, 0x84, 0x11, 0x34,
    0x14, 0x92, 0x1f, 0x2c, 0x0e, 0x0d, 0x14, 0x08, 0x09, 0x1c, 0x18, 0x07, 0x44, 0x86, 0x03, 0x90,
    0x40, 0xf2, 0x80, 0x0f, 0xdd, 0x23, 0xf4, 0xc1, 0x82, 0xb7, 0xc1, 0xc7, 0x34, 0x00, 0x11, 0x0a,
    0x11, 0x3c, 0x2a, 0x03, 0x02, 0x49, 0xa0, 0x23, 0x1e, 0x20, 0x33, 0x38, 0xe0, 0x10, 0x4b, 0x01,
    0x0c, 0xc5, 0x16, 0x16, 0xfc, 0x13, 0x90, 0xc0, 0xbf, 0x41, 0x05, 0x40, 0x3e, 0xfa, 0x08, 0x0e,
    0xa5, 0x06, 0x50, 0xa0, 0xfb, 0x81, 0x04, 0x99, 0x26, 0x12, 0xf7, 0x05, 0x7c, 0x40, 0x6b, 0x01,
    0x08, 0x85, 0x27, 0x0e, 0xd5, 0x0e, 0x0e, 0xb9, 0x02, 0x0a, 0x93, 0x07, 0x2a, 0xb2, 0x14, 0x94,
    0xe0, 0x9a, 0x60, 0x22, 0xbd, 0x60, 0x01, 0x5b, 0x80, 0x06, 0xe0, 0x27, 0x0a, 0x97, 0x0c, 0x32,
    0xd0, 0x01, 0x3a, 0xa4, 0x01, 0xa9, 0x14, 0xc8, 0xa2, 0x88, 0x84, 0x69, 0x84, 0x82, 0xb8, 0x81,
    0x82, 0x8f, 0x82, 0x75, 0x01, 0x1b, 0x3c, 0x11, 0xc1, 0x79, 0x06, 0x31, 0x2c, 0x0b, 0x80, 0x07,
    0xed, 0x81, 0x42, 0xf1, 0x00, 0x1e, 0x28, 0x22, 0xb8, 0xc4, 0x27, 0xc8, 0x1c, 0x17, 0x48, 0x9c,
    0x5b, 0x04, 0x56, 0x00, 0x0b, 0x48, 0x2f, 0x08, 0x01, 0x9b, 0x21, 0x91, 0x18, 0xb8, 0x20, 0x5e,
    0xe2, 0x22, 0xcc, 0xc0, 0x04, 0xb6, 0x06, 0x5c, 0xb0, 0x7a, 0x20, 0xa3, 0xba, 0x40, 0x05, 0xba,
    0x16, 0x88, 0xc0, 0x5b, 0x00, 0x09, 0xed, 0x08, 0x4c, 0x30, 0x7d, 0xe0, 0x80, 0x81, 0x08, 0xc0,
    0x06, 0x16, 0xe1, 0x09, 0x44, 0xa8, 0x0a, 0xc0, 0x0b, 0x80, 0x10, 0x1e, 0xa0, 0x00, 0x5b, 0xe0,
    0x60, 0xca, 0xe0, 0xa0, 0x8a, 0x40, 0x06, 0x9d, 0x35, 0x0a, 0x64, 0x70, 0xf0, 0xa1, 0xb1, 0xab,
    0xe1, 0xb3, 0x0a, 0x01, 0x09, 0xf8, 0x06, 0x12, 0x8e, 0x15, 0x48, 0xd0, 0x4b, 0x00, 0x05, 0x92,
    0x0e, 0x12, 0xc8, 0x1e, 0x60, 0x40, 0x3d, 0xe1, 0x40, 0x28, 0xc1, 0x0d, 0xa5, 0x03, 0x68, 0x60,
    0xc8, 0xc1, 0x0d, 0x85, 0x14, 0x3e, 0xcb, 0x0b, 0x1a, 0x8b, 0x02, 0x1e, 0xe1, 0x07, 0x3a, 0xf7,
    0x04, 0xac, 0xa0, 0x3f, 0xc0, 0x05, 0xf7, 0x18, 0x32, 0xb3, 0x06, 0x1a, 0x8c, 0x14, 0x26, 0xeb,
    0x05, 0x22, 0x9c, 0x19, 0x50, 0x80, 0xae, 0x90, 0x6f, 0x06, 0x15, 0x70, 0x07, 0x95, 0xea, 0x06,
    0x05, 0xe0, 0x04, 0x32, 0x50, 0x27, 0xd0, 0x69, 0xc6, 0xb0, 0x30, 0x6d, 0x40, 0x85, 0xfb, 0x02,
    0x8f, 0x66, 0x03, 0x85, 0x70, 0x01, 0xd1, 0x94, 0xd9, 0x2e, 0xb0, 0xb0, 0x34, 0x40, 0x84, 0x5a,
    0x01, 0x2a, 0x98, 0x26, 0xc8, 0xa8, 0x80, 0x45, 0x66, 0x84, 0x0e, 0x3e, 0x82, 0x4a, 0xb1, 0x43,
    0x2c, 0x2b, 0x64, 0xf6, 0x17, 0x3c, 0x24, 0x41, 0x54, 0x00, 0x17, 0x00, 0xc1, 0xb2, 0x82, 0x08,
    0x36, 0xd8, 0x00, 0x90, 0xc2, 0xc6, 0x3b, 0x42, 0x02, 0x32, 0x80, 0x0c, 0x8e, 0x0b, 0x80, 0x60,
    0x3a, 0x23, 0xd3, 0x16, 0x36, 0xd1, 0x1a, 0x0e, 0x80, 0x08, 0x0a, 0x81, 0x1a, 0x54, 0x80, 0x2c,
    0x80, 0x0c, 0xed, 0x1e, 0x44, 0xa0, 0x2a, 0x10, 0xf2, 0x17, 0x20, 0xc8, 0xb1, 0x90, 0x55, 0x90,
    0x68, 0x1d, 0xb0, 0x38, 0xc9, 0xea, 0x09, 0x17, 0x70, 0x77, 0x58, 0x50, 0x3f, 0x30, 0x01, 0x28,
    0x84, 0xc7, 0xb0, 0x81, 0xc7, 0xe1, 0x81, 0x8a, 0xa9, 0x85, 0x8c, 0x37, 0x81, 0x4f, 0xb3, 0x06,
    0x2a, 0x54, 0x3b, 0xb8, 0xf4, 0x0a, 0x50, 0x41, 0xed, 0x01, 0x17, 0xb8, 0x22, 0x90, 0x41, 0xb3,
    0x01, 0x10, 0x88, 0x2b, 0x20, 0x42, 0xea, 0x82, 0x04, 0xe0, 0x89, 0xc7, 0x64, 0x48, 0x8e, 0x1d,
    0x64, 0x3c, 0x07, 0x4c, 0xe6, 0x25, 0x14, 0x2c, 0x0d, 0x58, 0x01, 0x3f, 0x06, 0x0d, 0x9e, 0x0f,
    0x2c, 0xcc, 0x1f, 0x22, 0x2d, 0xa0, 0x52, 0xe3, 0x60, 0xaf, 0x00, 0x04, 0x91, 0x09, 0x3e, 0x88,
    0x04, 0xc1, 0x25, 0x90, 0xc8, 0x45, 0x51, 0xc1, 0x92, 0xc9, 0x00, 0x84, 0xcc, 0x03, 0x34, 0x88,
    0x1f, 0x60, 0x04, 0x7d, 0x03, 0x91, 0xc8, 0x00, 0x0d, 0x46, 0x01, 0x91, 0x66, 0x09, 0x87, 0x73,
    0x0b, 0x9b, 0x6c, 0x07, 0x17, 0xcd, 0x01, 0x9d, 0xc1, 0x80, 0xf4, 0x3f, 0x58, 0x18, 0x0e, 0x90,
    0xc1, 0x29, 0x8c, 0xc8, 0x26, 0x01, 0x1f, 0x80, 0x27, 0xc0, 0x04, 0x8a, 0xca, 0xe1, 0x83, 0x43,
    0x76, 0x41, 0x64, 0x21, 0x24, 0x50, 0x0d, 0x34, 0x5a, 0x07, 0xb2, 0x28, 0xa0, 0xf3, 0x2a, 0x00,
    0x04, 0xee, 0x06, 0x2e, 0xad, 0x15, 0x0a, 0x5b, 0x39, 0xa6, 0x80, 0x83, 0x63, 0x89, 0x84, 0x43,
    0xe8, 0xb0, 0x97, 0x58, 0x0c, 0x36, 0x78, 0x10, 0x0e, 0x30, 0x42, 0xe0, 0x02, 0x2d, 0x50, 0x3e,
    0x44, 0x30, 0xc1, 0x83, 0x11, 0x81, 0x10, 0xf2, 0x07, 0x64, 0xe6, 0x0f, 0x90, 0x60, 0x79, 0x81,
    0x0f, 0x62, 0x21, 0x1c, 0xc2, 0x07, 0x54, 0xa0, 0xb0, 0xa0, 0xcc, 0xe2, 0x31, 0x20, 0x99, 0x11,
    0x0e, 0x83, 0x16, 0x16, 0xe1, 0x1b, 0x70, 0xf0, 0x5e, 0x00, 0x0a, 0x9f, 0x1b, 0x1a, 0xf3, 0x1e,
    0x12, 0x8f, 0x05, 0x2e, 0x5f, 0x60, 0x00, 0x1d, 0x91, 0x6f, 0x09, 0x11, 0xeb, 0x0d, 0x0f, 0x58,
    0x0e, 0x4a, 0xa8, 0xd6, 0x70, 0x69, 0x1c, 0x20, 0x03, 0x7d, 0x13, 0x0f, 0x5d, 0x07, 0x54, 0x70,
    0x3c, 0x21, 0x85, 0x68, 0x15, 0x1b, 0x52, 0x06, 0x1f, 0xcc, 0x04, 0x87, 0xfb, 0x11, 0x95, 0x0c,
    0x05, 0xf6, 0x0c, 0x1b, 0x5f, 0x05, 0x05, 0x4e, 0x01, 0x8d, 0x7e, 0x05, 0x0b, 0xf5, 0x06, 0x28,
    0x90, 0x24, 0x90, 0x88, 0x0d, 0x00, 0x82, 0xcb, 0x06, 0x50, 0x78, 0x17, 0x70, 0x10, 0xdc, 0xb0,
    0x21, 0x16, 0x90, 0xf9, 0x4c, 0xa0, 0x82, 0xc2, 0x01, 0x30, 0x68, 0xcf, 0x30, 0x59, 0x45, 0x70,
    0xd8, 0x25, 0x40, 0x02, 0xfb, 0x0a, 0x07, 0xf9, 0x00, 0x07, 0x20, 0x0b, 0xd6, 0x07, 0x2a, 0xb0,
    0x3d, 0x00, 0x84, 0x20, 0x8f, 0xe5, 0x0a, 0x93, 0x44, 0x01, 0x2e, 0x68, 0x25, 0x11, 0x99, 0x53,
    0x10, 0x66, 0x10, 0x91, 0x1e, 0x50, 0xf8, 0x54, 0xd0, 0xe1, 0x26, 0x11, 0xb1, 0x7f, 0x20, 0x04,
    0x5b, 0x0b, 0x99, 0x7c, 0x0c, 0x09, 0xc8, 0x02, 0x17, 0x57, 0x0b, 0x0f, 0x26, 0x95, 0x73, 0x83,
    0x90, 0x1a, 0x00, 0x42, 0x67, 0x03, 0x1c, 0x70, 0x16, 0xd8, 0xb0, 0x4b, 0xd8, 0xc0, 0x3f, 0x40,
    0x01, 0xe8, 0x81, 0x02, 0xa4, 0x00, 0x22, 0xa0, 0x07, 0x78, 0xc4, 0x32, 0x68, 0xe4, 0x26, 0x30,
    0xc1, 0x66, 0x82, 0xc9, 0xe5, 0x02, 0x11, 0x80, 0x3b, 0xb8, 0xc4, 0x80, 0xc1, 0x34, 0x01, 0x1b,
    0xb8, 0x32, 0x68, 0x70, 0x7b, 0x70, 0x01, 0x72, 0x00, 0x10, 0x30, 0x0a, 0x10, 0x81, 0xed, 0x80,
    0x4f, 0xa4, 0x8a, 0x4a, 0x8b, 0xc5, 0xf6, 0x01, 0x4b, 0x4c, 0x0b, 0xe8, 0xd8, 0x17, 0xc0, 0x02,
    0xbd, 0x05, 0x11, 0xa9, 0xe0, 0x56, 0xc0, 0x43, 0xb6, 0x41, 0x42, 0x7a, 0x85, 0x0d, 0xc8, 0x25,
    0xf2, 0x1a, 0xe0, 0x30, 0x1e, 0x21, 0x61, 0x9b, 0x60, 0x53, 0x59, 0x60, 0xc3, 0x5c, 0x20, 0x31,
    0x89, 0xe1, 0x43, 0x38, 0xe0, 0x02, 0x3b, 0x02, 0x06, 0x84, 0x0d, 0x16, 0xe4, 0x0a, 0x40, 0x80,
    0x6a, 0x80, 0x19, 0x82, 0x14, 0x3e, 0xf7, 0x10, 0x4c, 0x00, 0x78, 0x00, 0x05, 0xa3, 0x36, 0x70,
    0x40, 0xef, 0x20, 0xe1, 0x28, 0xe1, 0x61, 0x2f, 0x90, 0x4d, 0x01, 0x87, 0x69, 0x08, 0x0b, 0xcb,
    0x06, 0x05, 0xc2, 0x04, 0x8f, 0x6b, 0x32, 0x22, 0x48, 0x46, 0x70, 0x18, 0x1e, 0x12, 0xc1, 0x51,
    0xc0, 0x71, 0x89, 0xaf, 0x08, 0xa8, 0x00, 0x10, 0x78, 0x16, 0x48, 0x0c, 0x16, 0xc4, 0x97, 0x80,
    0x12, 0xbc, 0x23, 0x38, 0x41, 0xd0, 0xc0, 0x83, 0x72, 0x00, 0x1e, 0x28, 0x0f, 0x6c, 0x70, 0x09,
    0xd8, 0xa0, 0x0d, 0x08, 0x04, 0x05, 0xa8, 0xc0, 0x50, 0xc1, 0xe1, 0x70, 0x00, 0x20, 0x68, 0x1b,
    0x24, 0x24, 0x25, 0xa8, 0x00, 0x91, 0x42, 0x81, 0x9a, 0xc1, 0x02, 0x32, 0x81, 0x09, 0xfa, 0x15,
    0x4c, 0xc4, 0x05, 0xb8, 0x20, 0x30, 0x01, 0x16, 0x1e, 0x0d, 0x64, 0xcc, 0xc8, 0xa0, 0xda, 0x01,
    0x08, 0x28, 0x29, 0x14, 0x0a, 0x23, 0x34, 0x58, 0x13, 0x34, 0xc8, 0x17, 0x3c, 0xe0, 0x07, 0x90,
    0x20, 0x1c, 0xc1, 0xc5, 0x30, 0x44, 0x83, 0xb9, 0x88, 0x09, 0xd2, 0x1f, 0x2c, 0x00, 0x1d, 0x14,
    0x52, 0x24, 0x5a, 0x0b, 0xf8, 0xa0, 0x34, 0x82, 0x21, 0x1e, 0x11, 0x1c, 0x22, 0x09, 0x44, 0x4c,
    0x2d, 0x24, 0x7c, 0x44, 0x28, 0x17, 0x54, 0x50, 0x13, 0x34, 0x5c, 0x07, 0xf0, 0xa0, 0x3a, 0x23,
    0xa8, 0x09, 0x54, 0x20, 0x7a, 0xa0, 0x91, 0x5b, 0x60, 0x33, 0x0d, 0xa1, 0x52, 0x5a, 0xc2, 0x05,
    0x80, 0x06, 0x0a, 0xd0, 0x06, 0x2e, 0xce, 0x0a, 0x40, 0xc0, 0x0f, 0x22, 0x53, 0x29, 0xe0, 0xe0,
    0x59, 0xa0, 0x90, 0x8f, 0xe0, 0xe1, 0x6a, 0xc0, 0x04, 0xe5, 0x04, 0x60, 0x50, 0x19, 0xc0, 0x07,
    0x93, 0x05, 0x1e, 0x3a, 0xc1, 0x0d, 0xb0, 0xe8, 0x5e, 0x10, 0xf9, 0xf3, 0xe1, 0x65, 0xf0, 0xe8,
    0x14, 0xb0, 0xb8, 0x67, 0xb0, 0x20, 0x84, 0xe0, 0x82, 0x5c, 0x03, 0x9b, 0xf8, 0x00, 0x05, 0xf2,
    0x05, 0x1b, 0x6d, 0x07, 0x26, 0x94, 0x44, 0x07, 0x80, 0x81, 0xb1, 0x05, 0x18, 0x14, 0x52, 0x50,
    0x81, 0xed, 0x86, 0x43, 0x3f, 0x01, 0x20, 0x98, 0x5e, 0xf8, 0xf8, 0x4b, 0x20, 0x81, 0xf6, 0x00,
    0x23, 0xb8, 0x0e, 0x44, 0x9f, 0x41, 0xc1, 0xdc, 0x40, 0xa1, 0xf4, 0xc0, 0x43, 0x30, 0x40, 0x82,
    0x55, 0x40, 0xa2, 0x71, 0x05, 0x1c, 0x16, 0x09, 0xb8, 0x60, 0x91, 0x20, 0xd3, 0x08, 0x26, 0xce,
    0x01, 0xf1, 0x3e, 0x20, 0x03, 0x38, 0x19, 0x47, 0x03, 0x9d, 0xca, 0x07, 0xd7, 0x11, 0x41, 0x15,
    0x90, 0x47, 0x98, 0x44, 0xa7, 0xf3, 0x40, 0x83, 0x58, 0x81, 0x16, 0x48, 0x03, 0xb8, 0xa0, 0x4f,
    0x44, 0xf9, 0xc0, 0x81, 0x32, 0x40, 0x81, 0x99, 0x04, 0x0a, 0xba, 0x3b, 0x50, 0x01, 0x3b, 0xc0,
    0x26, 0x37, 0x80, 0x10, 0xcc, 0x6d, 0xd0, 0xc0, 0x50, 0xc8, 0x47, 0x3d, 0x00, 0x0b, 0xba, 0x1d,
    0x4c, 0xa6, 0x29, 0x34, 0xbc, 0x07, 0x98, 0x40, 0x9a, 0x01, 0x17, 0xc6, 0x11, 0x24, 0x50, 0x03,
    0x5c, 0x70, 0x05, 0x64, 0xdc, 0x2f, 0x34, 0x5e, 0x2b, 0x00, 0x61, 0x7e, 0xc0, 0x85, 0x97, 0x00,
    0x0a, 0x54, 0x05, 0x92, 0x6b, 0xa0, 0xd0, 0xc9, 0x10, 0x49, 0x06, 0x85, 0xe6, 0x06, 0x07, 0x61,
    0x01, 0x32, 0x10, 0x93, 0xf8, 0x2d, 0x10, 0xe1, 0x65, 0x70, 0xb0, 0x47, 0x70, 0xb1, 0x74, 0xa0,
    0x82, 0x59, 0x05, 0x32, 0x48, 0x26, 0x90, 0x01, 0x3c, 0xf0, 0x80, 0x51, 0xb0, 0x52, 0x09, 0x0e,
    0xb0, 0x41, 0xde, 0x70, 0x91, 0xd5, 0x48, 0xad, 0x80, 0x02, 0x32, 0x83, 0x0e, 0xe5, 0x85, 0x43,
    0xa0, 0x00, 0x14, 0x74, 0x16, 0x24, 0xd7, 0x40, 0x46, 0x70, 0x40, 0x63, 0x74, 0x41, 0xa6, 0x79,
    0x00, 0x08, 0xfe, 0x15, 0xb0, 0xe0, 0x18, 0x81, 0x0d, 0xb2, 0x03, 0xd0, 0x61, 0x9e, 0x83, 0x0c,
    0xc2, 0x2d, 0xb0, 0x80, 0xb0, 0xc1, 0xe1, 0xf4, 0xc1, 0x42, 0x35, 0xc0, 0x23, 0x71, 0xc1, 0xc1,
    0xbf, 0x06, 0x0a, 0xe6, 0x1b, 0x54, 0xde, 0x19, 0x44, 0xe8, 0x4c, 0x60, 0x05, 0xa2, 0x2d, 0xa0,
    0x20, 0xaf, 0x60, 0x23, 0x8e, 0x80, 0x08, 0xd6, 0x09, 0x78, 0x70, 0x5f, 0x81, 0x08, 0x96, 0x06,
    0x2a, 0xf8, 0x0b, 0x12, 0xf8, 0x03, 0x0a, 0xfb, 0x12, 0x1a, 0x80, 0x01, 0x32, 0x90, 0x09, 0x2e,
    0x8a, 0x16, 0x3a, 0xe5, 0x36, 0x3e, 0xd1, 0x09, 0x2a, 0xf2, 0x02, 0x22, 0x61, 0x40, 0xf0, 0x5c,
    0x20, 0x61, 0x9c, 0x20, 0xf1, 0x3d, 0xa0, 0x60, 0x2f, 0x20, 0x63, 0x78, 0xc0, 0x05, 0xce, 0x0d,
    0x26, 0xe9, 0x02, 0x44, 0xf0, 0xad, 0x20, 0x52, 0x7f, 0xe0, 0x33, 0x2b, 0xa0, 0x93, 0x9c, 0xa0,
    0xb0, 0x29, 0xa0, 0xa0, 0xb8, 0x20, 0x41, 0xf9, 0x60, 0x92, 0x1e, 0x61, 0x12, 0x2a, 0x00, 0x05,
    0xc9, 0x03, 0x16, 0xd0, 0x01, 0x5c, 0x11, 0x38, 0x90, 0x72, 0x01, 0x32, 0x68, 0x37, 0x60, 0x02,
    0xe3, 0x07, 0x22, 0x00, 0x9d, 0x88, 0x6b, 0x40, 0x24, 0x11, 0x4c, 0x4a, 0x1b, 0x3c, 0x92, 0x07,
    0x7c, 0x1c, 0x2b, 0xa0, 0x40, 0x98, 0x40, 0xa5, 0x39, 0x40, 0x42, 0x88, 0x0f, 0x92, 0x0f, 0x7c,
    0xae, 0x23, 0xa0, 0x61, 0x1e, 0xc1, 0xc1, 0xd7, 0xc2, 0x43, 0x57, 0x40, 0xc6, 0xba, 0x00, 0x0a,
    0xa0, 0x0b, 0x20, 0xc1, 0x75, 0x23, 0xd6, 0x0a, 0x3e, 0xbd, 0x04, 0xac, 0x90, 0xbc, 0xa0, 0xf2,
    0x64, 0xa2, 0x1e, 0x40, 0x0b, 0xe2, 0x07, 0x70, 0xd0, 0xa8, 0xe0, 0x22, 0x26, 0x81, 0xa8, 0x20,
    0x11, 0x0d, 0x61, 0xf1, 0x4e, 0xa1, 0xb0, 0x07, 0x04, 0x5a, 0x1e, 0x83, 0x05, 0xe1, 0x57, 0xd0,
    0xf8, 0x6e, 0x30, 0x91, 0x3c, 0x60, 0x82, 0xe5, 0x0b, 0x2a, 0xf0, 0x9f, 0xe0, 0x04,
];
