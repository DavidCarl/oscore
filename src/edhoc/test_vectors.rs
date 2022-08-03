


pub const SUITE_I: u8 = 0;
pub const METHOD_TYPE_I : u8 = 3;
pub const _I_EPHEMERAL_PK:[u8; 32] = [0x3A, 0xA9, 0xEB, 0x32, 0x01, 0xB3, 0x36, 0x7B, 0x8C, 
                            0x8B, 0xE3, 0x8D, 0x91, 0xE5, 0x7A, 0x2B, 0x43, 0x3E,
                            0x67, 0x88, 0x8C, 0x86, 0xD2,0xAC, 0x00, 0x6A, 0x52, 
                            0x08, 0x42, 0xED, 0x50, 0x37];
pub const KID_I : [u8;1] = [5];
pub const I_STATIC_PK : [u8;32] = [0x4A,0x49,0xD8,0x8C,0xD5,0xD8,0x41,0xFA,0xB7,0xEF,0x98,
                                   0x3E,0x91,0x1D,0x25,0x78,0x86,0x1F,0x95,0x88,0x4F,0x9F,
                                   0x5D,0xC4,0x2A,0x2E,0xED,0x33,0xDE,0x79,0xED,0x77];
pub const I_STATIC_SK : [u8;32] = [0xCF,0xC4,0xB6,0xED,0x22,0xE7,0x00,0xA3,0x0D,0x5C,0x5B,
                                   0xCD,0x61,0xF1,0xF0,0x20,0x49,0xDE,0x23,0x54,0x62,0x33,
                                   0x48,0x93,0xD6,0xFF,0x9F,0x0C,0xFE,0xA3,0xFE,0x04];

pub const _R_STATIC_PK : [u8;32]= [0xE6,0x6F,0x35,0x59,0x90,0x22,0x3C,0x3F,0x6C,0xAF,0xF8,
                                  0x62,0xE4,0x07,0xED,0xD1,0x17,0x4D,0x07,0x01,0xA0,0x9E,
                                  0xCD,0x6A,0x15,0xCE,0xE2,0xC6,0xCE,0x21,0xAA,0x50];

pub const MSG1 :[u8;38] = [0x03,0x00,0x58,0x20,0x3A,0xA9,0xEB,0x32,0x01,0xB3,0x36,0x7B,0x8C,
                            0x8B,0xE3,0x8D,0x91,0xE5,0x7A,0x2B,0x43,0x3E,0x67,0x88,0x8C,0x86,
                            0xD2,0xAC,0x00,0x6A,0x52,0x08,0x42,0xED,0x50,0x37,0x41,0x0C];

pub const R_EPHEMERAL_PK:  [u8;32] = [0x25,0x54,0x91,0xB0,0x5A,0x39,
0x89,0xFF,0x2D,0x3F,0xFE,0xA6,0x20,0x98,0xAA,0xB5,0x7C,0x16,0x0F,0x29,
0x4E,0xD9,0x48,0x01,0x8B,0x41,0x90,0xF7,0xD1,0x61,0x82,0x4E];
pub const C_R : [u8;1] = [0x40];
pub const C_I : [u8;1] = [0xC];



pub const I_EPHEMEREAL_SK : [u8;32] = [0xB3,0x11,0x19,0x98,0xCB,0x3F,0x66,0x86,0x63,0xED,0x42,0x51,
                            0xC7,0x8B,0xE6,0xE9,0x5A,0x4D,0xA1,0x27,0xE4,0xF6,0xFE,0xE2,
                            0x75,0xE8,0x55,0xD8,0xD9,0xDF,0xD8,0xED];



pub const TH_2_RAW_INPUT :[u8;69] = [0x58,0x20,0x9B,0xDD,0xB0,0xCD,0x55,0x48,0x7F,0x82,0xA8,
                            0x6F,0xB7,0x2A,0x8B,0xB3,0x58,0x52,0x68,0x91,0xA0,0xA6,
                            0xC9,0x08,0x61,0x24,0x12,0xF5,0xAF,0x29,0x9D,0xAF,0x01,
                            0x96,0x58,0x20,0x25,0x54,0x91,0xB0,0x5A,0x39,0x89,0xFF,
                            0x2D,0x3F,0xFE,0xA6,0x20,0x98,0xAA,0xB5,0x7C,0x16,0x0F,
                            0x29,0x4E,0xD9,0x48,0x01,0x8B,0x41,0x90,0xF7,0xD1,0x61,
                            0x82,0x4E,0x40];

pub const TH_2_CBOR : [u8;34] =[0x58,0x20,0x71,0xA6,0xC7,0xC5,0xBA,0x9A,0xD4,0x7F,
                           0xE7,0x2D,0xA4,0xDC,0x35,0x9B,0xF6,0xB2,0x76,0xD3,
                           0x51,0x59,0x68,0x71,0x1B,0x9A,0x91,0x1C,0x71,0xFC,
                           0x09,0x6A,0xEE,0x0E];
pub const TH_2 : [u8;32] = [0x71,0xA6,0xC7,0xC5,0xBA,0x9A,0xD4,0x7F,
                                0xE7,0x2D,0xA4,0xDC,0x35,0x9B,0xF6,0xB2,
                                0x76,0xD3,0x51,0x59,0x68,0x71,0x1B,0x9A,
                                0x91,0x1C,0x71,0xFC,0x09,0x6A,0xEE,0x0E];



pub const TH_4_CBOR : [u8;34] = [0x58,0x20,0x4B,0x9A,0xDD,0x2A,0x9E,0xEB,
                                0x88,0x49,0x71,0x6C,0x79,0x68,0x78,0x4F,
                                0x55,0x40,0xDD,0x64,0xA3,0xBB,0x07,0xF8,
                                0xD0,0x00,0xAD,0xCE,0x88,0xB6,0x30,0xD8,
                                0x84,0xEB];





pub const CRED_R : [u8;59]= [0xA2,0x02,0x6B,0x65,0x78,0x61,0x6D,0x70,0x6C,0x65,0x2E,
                    0x65,0x64,0x75,0x08,0xA1,0x01,0xA4,0x01,0x01,0x02,0x05,
                    0x20,0x04,0x21,0x58,0x20,0xE6,0x6F,0x35,0x59,0x90,0x22,
                    0x3C,0x3F,0x6C,0xAF,0xF8,0x62,0xE4,0x07,0xED,0xD1,0x17,
                    0x4D,0x07,0x01,0xA0,0x9E,0xCD,0x6A,0x15,0xCE,0xE2,0xC6,
                    0xCE,0x21,0xAA,0x50];

pub const CIPHERTEXT_2 : [u8;10] = [0x0F,0xF0,0x4C,0x29,0x4F,0x4A,0xC6,0x02,0xCF,0x78];

pub const CIPHERTEXT_3 : [u8;18] = [0xBE,0x01,0x46,0xC1,0x36,0xAC,0x2E,0xFF,0xD4,0x53,0xA7,0x5E,0xFA,0x90,0x89,0x6F,0x65,0x3B];
pub const ID_CRED_R : [u8;3]= [0xA1, 0x04, 0x05];

pub const IV_3 : [u8;13] = [0xB3,0x8F,0xB6,0x31,0xE3,0x44,0xA8,0x10,0x52,0x56,0x32,0xED,0xF8];

pub const A_3 :[u8;45] = [0x83,0x68,0x45,0x6E,0x63,0x72,0x79,0x70,0x74,0x30,0x40,0x58,0x20,0xA4,
                          0x90,0x07,0xCE,0x54,0x76,0x2E,0x46,0x7C,0x4E,0x4A,0x44,0x69,0x2F,0x20,
                          0x70,0xD3,0xE9,0xEB,0x00,0xF9,0x5A,0xC2,0x62,0x9B,0x2B,0xBE,0xF7,0xFB,
                          0x24,0xA3,0x70];

pub const P_3 :[u8;10] = [0x29,0x48,0xDB,0x0B,0x8F,0x75,0x27,0x09,0x53,0xDA];

pub const K_3 : [u8;16] = [0x2A,0x30,0xE4,0xF6,0xBC,0x55,0x8D,0x0E,0x7A,0x8C,0x63,0xEE,0x7B,0xB5,0x45,0x7F];

pub const MSG2 : [u8;46] = [0x58,0x2A,0x25,0x54,0x91,0xB0,0x5A,0x39,
                            0x89,0xFF,0x2D,0x3F,0xFE,0xA6,0x20,0x98,
                            0xAA,0xB5,0x7C,0x16,0x0F,0x29,0x4E,0xD9,
                            0x48,0x01,0x8B,0x41,0x90,0xF7,0xD1,0x61,
                            0x82,0x4E,0x0F,0xF0,0x4C,0x29,0x4F,0x4A,
                            0xC6,0x02,0xCF,0x78,0x41,0x40];

pub const SHARED_SECRET_0 :[u8;32]= [0x6D,0x26,0x60,0xEC,0x2B,0x30,0x15,0xD9,0x3F,
                            0xE6,0x5D,0xAE,0xA5,0x12,0x74,0xBD,0x5B,0x1E,
                            0xBB,0xAD,0x9B,0x62,0x4E,0x67,0x0E,0x79,0xA6,
                            0x55,0xE3,0x0E,0xC3,0x4D];

pub const SHARED_SECRET_1 :[u8;32]=[0xB5,0x8B,0x40,0x34,0x26,0xC0,0x3D,0xB0,0x7B,
                                    0xAA,0x93,0x44,0xD5,0x51,0xE6,0x7B,0x21,0x78,
                                    0xBF,0x05,0xEC,0x6F,0x52,0xC3,0x6A,0x2F,0xA5,
                                    0xBE,0x23,0x2D,0xD4,0x78];


pub const SHARED_SECRET_2 :[u8;32]=[0x0A,0xF4,0x2A,0xD5,0x12,0xDC,0x3E,0x97,0x2B,
                            0x3A,0xC4,0xD4,0x7B,0xA3,0x3F,0xFC,0x21,0xF1,0xAE,0x6F,
                            0x07,0xF2,0xF8,0x94,0x85,0x4A,0x5A,0x47,0x44,0x33,0x85,0x48];

pub const PRK2E : [u8;32] = [0xD1,0xD0,0x11,0xA5,0x9A,0x6D,0x10,0x57,0x5E,
                        0xB2,0x20,0xC7,0x65,0x2E,0x6F,0x98,0xC4,0x17,0xA5,
                        0x65,0xE4,0xE4,0x5C,0xF5,0xB5,0x01,0x06,0x95,0x04,
                        0x3B,0x0E,0xB7];
pub const PRK3EM : [u8;32] = [0x76,0x8E,0x13,0x75,0x27,0x2E,0x1E,0x68,0xB4,
                            0x2C,0xA3,0x24,0x84,0x80,0xD5,0xBB,0xA8,0x8B,
                            0xCB,0x55,0xF6,0x60,0xCE,0x7F,0x94,0x1E,0x67,
                            0x09,0x10,0x31,0x17,0xA1];
pub const PRK4XM : [u8;32]=[0xB8,0xCC,0xDF,0x14,0x20,0xB5,0xB0,0xC8,0x2A,
                            0x58,0x7E,0x7D,0x26,0xDD,0x7B,0x70,0x48,0x57
                            ,0x4C,0x3A,0x48,0xDF,0x9F,0x6A,0x45,0xF7,0x21,
                            0xC0,0xCF,0xA4,0xB2,0x7C];


pub const MAC_2 : [u8;8] = [0x8E,0x27,0xCB,0xD4,0x94,0xF7,0x52,0x83];


pub const MASTER_SECRET : [u8;16] =[0xC0,0x53,0x01,0x37,0x6C,0xE9,0x5F,0x67,0xC4,0x14,0xD8,0xBB,0x5F,0x0F,0xDB,0x5E];

pub const PLAINTEXT_2 : [u8;11] = [0x41,0x05,0x48,0x8E,0x27,0xCB,0xD4,0x94,0xF7,0x52,0x83];
pub const _MASTER_SECRET_INFO :[u8;57]= [0x58,0x20,0x4B,0x9A,0xDD,0x2A,0x9E,0xEB,0x88,
                                        0x49,0x71,0x6C,0x79,0x68,0x78,0x4F,0x55,0x40,
                                        0xDD,0x64,0xA3,0xBB,0x07,0xF8,0xD0,0x00,0xAD,
                                        0xCE,0x88,0xB6,0x30,0xD8,0x84,0xEB,0x74,0x4F,
                                        0x53,0x43,0x4F,0x52,0x45,0x5F,0x4D,0x61,0x73,
                                        0x74,0x65,0x72,0x5F,0x53,0x65,0x63,0x72,0x65,
                                        0x74,0x40,0x10];