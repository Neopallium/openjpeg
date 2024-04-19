/*
 * The copyright in this software is being made available under the 2-clauses
 * BSD License, included below. This software may be subject to other third
 * party and contributor rights, including patent rights, and no such rights
 * are granted under this license.
 *
 * Copyright (c) 2005, Herve Drolon, FreeImage Team
 * Copyright (c) 2008, 2011-2012, Centre National d'Etudes Spatiales (CNES), FR
 * Copyright (c) 2012, CS Systemes d'Information, France
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS `AS IS'
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */
use super::j2k::*;
use super::jp2::*;

use super::c_api_types::*;
use super::consts::*;
use super::types::*;

use super::event::opj_event_mgr;

#[cfg(feature = "file-io")]
use ::libc::FILE;

pub(crate) enum CodecFormat {
  J2K(opj_j2k),
  JP2(opj_jp2),
}

pub(crate) enum Codec {
  Encoder(CodecFormat),
  Decoder(CodecFormat),
}

#[repr(C)]
pub(crate) struct opj_codec_private {
  pub m_codec: Codec,
  pub m_event_mgr: opj_event_mgr,
}
pub(crate) type opj_codec_private_t = opj_codec_private;

impl opj_codec_private {
  pub fn set_info_handler(
    &mut self,
    mut p_callback: opj_msg_callback,
    mut p_user_data: *mut core::ffi::c_void,
  ) -> OPJ_BOOL {
    self.m_event_mgr.set_info_handler(p_callback, p_user_data);
    1i32
  }

  pub fn set_warning_handler(
    &mut self,
    mut p_callback: opj_msg_callback,
    mut p_user_data: *mut core::ffi::c_void,
  ) -> OPJ_BOOL {
    self
      .m_event_mgr
      .set_warning_handler(p_callback, p_user_data);
    1i32
  }

  pub fn set_error_handler(
    &mut self,
    mut p_callback: opj_msg_callback,
    mut p_user_data: *mut core::ffi::c_void,
  ) -> OPJ_BOOL {
    self.m_event_mgr.set_error_handler(p_callback, p_user_data);
    1i32
  }

  pub unsafe fn set_threads(&mut self, mut num_threads: core::ffi::c_int) -> OPJ_BOOL {
    if num_threads >= 0i32 {
      match &mut self.m_codec {
        Codec::Encoder(CodecFormat::J2K(j2k)) | Codec::Decoder(CodecFormat::J2K(j2k)) => {
          opj_j2k_set_threads(j2k, num_threads as _)
        }
        Codec::Encoder(CodecFormat::JP2(jp2)) | Codec::Decoder(CodecFormat::JP2(jp2)) => {
          opj_jp2_set_threads(jp2, num_threads as _)
        }
      }
    } else {
      0
    }
  }

  #[cfg(feature = "file-io")]
  pub unsafe fn dump_codec(&mut self, mut info_flag: OPJ_INT32, mut output_stream: *mut FILE) {
    match &mut self.m_codec {
      Codec::Encoder(CodecFormat::J2K(j2k)) | Codec::Decoder(CodecFormat::J2K(j2k)) => {
        j2k_dump(j2k, info_flag, output_stream)
      }
      Codec::Encoder(CodecFormat::JP2(jp2)) | Codec::Decoder(CodecFormat::JP2(jp2)) => {
        jp2_dump(jp2, info_flag, output_stream)
      }
    }
  }

  pub unsafe fn get_cstr_info(&mut self) -> *mut opj_codestream_info_v2_t {
    match &mut self.m_codec {
      Codec::Encoder(CodecFormat::J2K(j2k)) | Codec::Decoder(CodecFormat::J2K(j2k)) => {
        j2k_get_cstr_info(j2k)
      }
      Codec::Encoder(CodecFormat::JP2(jp2)) | Codec::Decoder(CodecFormat::JP2(jp2)) => {
        jp2_get_cstr_info(jp2)
      }
    }
  }

  pub unsafe fn get_cstr_index(&mut self) -> *mut opj_codestream_index_t {
    match &mut self.m_codec {
      Codec::Encoder(CodecFormat::J2K(j2k)) | Codec::Decoder(CodecFormat::J2K(j2k)) => {
        j2k_get_cstr_index(j2k)
      }
      Codec::Encoder(CodecFormat::JP2(jp2)) | Codec::Decoder(CodecFormat::JP2(jp2)) => {
        jp2_get_cstr_index(jp2)
      }
    }
  }
}

// Decoder
impl opj_codec_private {
  pub unsafe fn setup_decoder(&mut self, mut parameters: *mut opj_dparameters_t) -> OPJ_BOOL {
    match &mut self.m_codec {
      Codec::Encoder(_) => {
        event_msg!(
          &mut self.m_event_mgr,
          EVT_ERROR,
          "Codec provided to the opj_setup_decoder function is not a decompressor handler.\n",
        );
      }
      Codec::Decoder(dec) => {
        if !parameters.is_null() {
          match dec {
            CodecFormat::J2K(dec) => {
              opj_j2k_setup_decoder(dec, parameters);
            }
            CodecFormat::JP2(dec) => {
              opj_jp2_setup_decoder(dec, parameters);
            }
          }
          return 1;
        }
      }
    }
    0i32
  }

  pub unsafe fn decoder_set_strict_mode(&mut self, mut strict: OPJ_BOOL) -> OPJ_BOOL {
    match &mut self.m_codec {
      Codec::Encoder(_) => {
        event_msg!(&mut self.m_event_mgr,
                      EVT_ERROR,
                      "Codec provided to the opj_decoder_set_strict_mode function is not a decompressor handler.\n",);
        0
      }
      Codec::Decoder(dec) => {
        match dec {
          CodecFormat::J2K(dec) => {
            opj_j2k_decoder_set_strict_mode(dec, strict);
          }
          CodecFormat::JP2(dec) => {
            opj_jp2_decoder_set_strict_mode(dec, strict);
          }
        }
        1
      }
    }
  }

  pub unsafe fn read_header(
    &mut self,
    mut p_stream: *mut opj_stream_t,
    mut p_image: *mut *mut opj_image_t,
  ) -> OPJ_BOOL {
    match &mut self.m_codec {
      Codec::Encoder(_) => {
        event_msg!(
          &mut self.m_event_mgr,
          EVT_ERROR,
          "Codec provided to the opj_read_header function is not a decompressor handler.\n",
        );
      }
      Codec::Decoder(dec) => {
        if !p_stream.is_null() {
          let mut l_stream = p_stream as *mut opj_stream_private_t;
          return match dec {
            CodecFormat::J2K(dec) => {
              opj_j2k_read_header(l_stream, dec, p_image, &mut self.m_event_mgr)
            }
            CodecFormat::JP2(dec) => {
              opj_jp2_read_header(l_stream, dec, p_image, &mut self.m_event_mgr)
            }
          };
        }
      }
    }
    0
  }

  pub unsafe fn set_decoded_components(
    &mut self,
    mut numcomps: OPJ_UINT32,
    mut comps_indices: *const OPJ_UINT32,
    mut apply_color_transforms: OPJ_BOOL,
  ) -> OPJ_BOOL {
    match &mut self.m_codec {
      Codec::Encoder(_) => {
        event_msg!(&mut self.m_event_mgr,
                      EVT_ERROR,
                      "Codec provided to the opj_set_decoded_components function is not a decompressor handler.\n",
                      );
        0
      }
      Codec::Decoder(dec) => {
        if apply_color_transforms != 0 {
          event_msg!(
            &mut self.m_event_mgr,
            EVT_ERROR,
            "apply_color_transforms = OPJ_TRUE is not supported.\n",
          );
          return 0i32;
        }
        match dec {
          CodecFormat::J2K(dec) => {
            opj_j2k_set_decoded_components(dec, numcomps, comps_indices, &mut self.m_event_mgr)
          }
          CodecFormat::JP2(dec) => {
            opj_jp2_set_decoded_components(dec, numcomps, comps_indices, &mut self.m_event_mgr)
          }
        }
      }
    }
  }

  pub unsafe fn decode(
    &mut self,
    mut p_stream: *mut opj_stream_t,
    mut p_image: *mut opj_image_t,
  ) -> OPJ_BOOL {
    match &mut self.m_codec {
      Codec::Encoder(_) => (),
      Codec::Decoder(dec) => {
        if !p_stream.is_null() {
          let mut l_stream = p_stream as *mut opj_stream_private_t;
          return match dec {
            CodecFormat::J2K(dec) => opj_j2k_decode(dec, l_stream, p_image, &mut self.m_event_mgr),
            CodecFormat::JP2(dec) => opj_jp2_decode(dec, l_stream, p_image, &mut self.m_event_mgr),
          };
        }
      }
    }
    0i32
  }

  pub unsafe fn end_decompress(&mut self, mut p_stream: *mut opj_stream_t) -> OPJ_BOOL {
    match &mut self.m_codec {
      Codec::Encoder(_) => (),
      Codec::Decoder(dec) => {
        if !p_stream.is_null() {
          let mut l_stream = p_stream as *mut opj_stream_private_t;
          return match dec {
            CodecFormat::J2K(dec) => opj_j2k_end_decompress(dec, l_stream, &mut self.m_event_mgr),
            CodecFormat::JP2(dec) => opj_jp2_end_decompress(dec, l_stream, &mut self.m_event_mgr),
          };
        }
      }
    }
    0
  }

  pub unsafe fn set_decode_area(
    &mut self,
    mut p_image: *mut opj_image_t,
    mut p_start_x: OPJ_INT32,
    mut p_start_y: OPJ_INT32,
    mut p_end_x: OPJ_INT32,
    mut p_end_y: OPJ_INT32,
  ) -> OPJ_BOOL {
    match &mut self.m_codec {
      Codec::Encoder(_) => (),
      Codec::Decoder(dec) => {
        return match dec {
          CodecFormat::J2K(dec) => opj_j2k_set_decode_area(
            dec,
            p_image,
            p_start_x,
            p_start_y,
            p_end_x,
            p_end_y,
            &mut self.m_event_mgr,
          ),
          CodecFormat::JP2(dec) => opj_jp2_set_decode_area(
            dec,
            p_image,
            p_start_x,
            p_start_y,
            p_end_x,
            p_end_y,
            &mut self.m_event_mgr,
          ),
        };
      }
    }
    0i32
  }

  pub unsafe fn read_tile_header(
    &mut self,
    mut p_stream: *mut opj_stream_t,
    mut p_tile_index: *mut OPJ_UINT32,
    mut p_data_size: *mut OPJ_UINT32,
    mut p_tile_x0: *mut OPJ_INT32,
    mut p_tile_y0: *mut OPJ_INT32,
    mut p_tile_x1: *mut OPJ_INT32,
    mut p_tile_y1: *mut OPJ_INT32,
    mut p_nb_comps: *mut OPJ_UINT32,
    mut p_should_go_on: *mut OPJ_BOOL,
  ) -> OPJ_BOOL {
    match &mut self.m_codec {
      Codec::Encoder(_) => (),
      Codec::Decoder(dec) => {
        if !p_stream.is_null() && !p_data_size.is_null() && !p_tile_index.is_null() {
          let mut l_stream = p_stream as *mut opj_stream_private_t;
          return match dec {
            CodecFormat::J2K(dec) => opj_j2k_read_tile_header(
              dec,
              p_tile_index,
              p_data_size,
              p_tile_x0,
              p_tile_y0,
              p_tile_x1,
              p_tile_y1,
              p_nb_comps,
              p_should_go_on,
              l_stream,
              &mut self.m_event_mgr,
            ),
            CodecFormat::JP2(dec) => opj_jp2_read_tile_header(
              dec,
              p_tile_index,
              p_data_size,
              p_tile_x0,
              p_tile_y0,
              p_tile_x1,
              p_tile_y1,
              p_nb_comps,
              p_should_go_on,
              l_stream,
              &mut self.m_event_mgr,
            ),
          };
        }
      }
    }
    0i32
  }

  pub unsafe fn decode_tile_data(
    &mut self,
    mut p_stream: *mut opj_stream_t,
    mut p_tile_index: OPJ_UINT32,
    mut p_data: *mut OPJ_BYTE,
    mut p_data_size: OPJ_UINT32,
  ) -> OPJ_BOOL {
    match &mut self.m_codec {
      Codec::Encoder(_) => (),
      Codec::Decoder(dec) => {
        if !p_data.is_null() && !p_stream.is_null() {
          let mut l_stream = p_stream as *mut opj_stream_private_t;
          return match dec {
            CodecFormat::J2K(dec) => opj_j2k_decode_tile(
              dec,
              p_tile_index,
              p_data,
              p_data_size,
              l_stream,
              &mut self.m_event_mgr,
            ),
            CodecFormat::JP2(dec) => opj_jp2_decode_tile(
              dec,
              p_tile_index,
              p_data,
              p_data_size,
              l_stream,
              &mut self.m_event_mgr,
            ),
          };
        }
      }
    }
    0i32
  }

  pub unsafe fn get_decoded_tile(
    &mut self,
    mut p_stream: *mut opj_stream_t,
    mut p_image: *mut opj_image_t,
    mut tile_index: OPJ_UINT32,
  ) -> OPJ_BOOL {
    match &mut self.m_codec {
      Codec::Encoder(_) => (),
      Codec::Decoder(dec) => {
        if !p_stream.is_null() {
          let mut l_stream = p_stream as *mut opj_stream_private_t;
          return match dec {
            CodecFormat::J2K(dec) => {
              opj_j2k_get_tile(dec, l_stream, p_image, &mut self.m_event_mgr, tile_index)
            }
            CodecFormat::JP2(dec) => {
              opj_jp2_get_tile(dec, l_stream, p_image, &mut self.m_event_mgr, tile_index)
            }
          };
        }
      }
    }
    0
  }

  pub unsafe fn set_decoded_resolution_factor(&mut self, mut res_factor: OPJ_UINT32) -> OPJ_BOOL {
    match &mut self.m_codec {
      Codec::Encoder(_) => 0,
      Codec::Decoder(CodecFormat::J2K(dec)) => {
        opj_j2k_set_decoded_resolution_factor(dec, res_factor, &mut self.m_event_mgr)
      }
      Codec::Decoder(CodecFormat::JP2(dec)) => {
        opj_jp2_set_decoded_resolution_factor(dec, res_factor, &mut self.m_event_mgr)
      }
    }
  }
}

// Encoder
impl opj_codec_private {
  pub unsafe fn setup_encoder(
    &mut self,
    mut parameters: *mut opj_cparameters_t,
    mut p_image: *mut opj_image_t,
  ) -> OPJ_BOOL {
    match &mut self.m_codec {
      Codec::Encoder(enc) => {
        if !parameters.is_null() && !p_image.is_null() {
          return match enc {
            CodecFormat::J2K(enc) => {
              opj_j2k_setup_encoder(enc, parameters, p_image, &mut self.m_event_mgr)
            }
            CodecFormat::JP2(enc) => {
              opj_jp2_setup_encoder(enc, parameters, p_image, &mut self.m_event_mgr)
            }
          };
        }
      }
      Codec::Decoder(_) => (),
    }
    0i32
  }

  pub unsafe fn encoder_set_extra_options(
    &mut self,
    mut options: *const *const core::ffi::c_char,
  ) -> OPJ_BOOL {
    match &mut self.m_codec {
      Codec::Encoder(enc) => match enc {
        CodecFormat::J2K(enc) => {
          opj_j2k_encoder_set_extra_options(enc, options, &mut self.m_event_mgr)
        }
        CodecFormat::JP2(enc) => {
          opj_jp2_encoder_set_extra_options(enc, options, &mut self.m_event_mgr)
        }
      },
      Codec::Decoder(_) => 0,
    }
  }

  pub unsafe fn start_compress(
    &mut self,
    mut p_image: *mut opj_image_t,
    mut p_stream: *mut opj_stream_t,
  ) -> OPJ_BOOL {
    match &mut self.m_codec {
      Codec::Encoder(enc) => {
        if !p_stream.is_null() {
          let mut l_stream = p_stream as *mut opj_stream_private_t;
          return match enc {
            CodecFormat::J2K(enc) => {
              opj_j2k_start_compress(enc, l_stream, p_image, &mut self.m_event_mgr)
            }
            CodecFormat::JP2(enc) => {
              opj_jp2_start_compress(enc, l_stream, p_image, &mut self.m_event_mgr)
            }
          };
        }
      }
      Codec::Decoder(_) => (),
    }
    0i32
  }

  pub unsafe fn encode(&mut self, mut p_stream: *mut opj_stream_t) -> OPJ_BOOL {
    match &mut self.m_codec {
      Codec::Encoder(enc) => {
        if !p_stream.is_null() {
          let mut l_stream = p_stream as *mut opj_stream_private_t;
          return match enc {
            CodecFormat::J2K(enc) => opj_j2k_encode(enc, l_stream, &mut self.m_event_mgr),
            CodecFormat::JP2(enc) => opj_jp2_encode(enc, l_stream, &mut self.m_event_mgr),
          };
        }
      }
      Codec::Decoder(_) => (),
    }
    0i32
  }

  pub unsafe fn end_compress(&mut self, mut p_stream: *mut opj_stream_t) -> OPJ_BOOL {
    match &mut self.m_codec {
      Codec::Encoder(enc) => {
        if !p_stream.is_null() {
          let mut l_stream = p_stream as *mut opj_stream_private_t;
          return match enc {
            CodecFormat::J2K(enc) => opj_j2k_end_compress(enc, l_stream, &mut self.m_event_mgr),
            CodecFormat::JP2(enc) => opj_jp2_end_compress(enc, l_stream, &mut self.m_event_mgr),
          };
        }
      }
      Codec::Decoder(_) => (),
    }
    0i32
  }

  pub unsafe fn write_tile(
    &mut self,
    mut p_tile_index: OPJ_UINT32,
    mut p_data: *mut OPJ_BYTE,
    mut p_data_size: OPJ_UINT32,
    mut p_stream: *mut opj_stream_t,
  ) -> OPJ_BOOL {
    match &mut self.m_codec {
      Codec::Encoder(enc) => {
        if !p_stream.is_null() && !p_data.is_null() {
          let mut l_stream = p_stream as *mut opj_stream_private_t;
          return match enc {
            CodecFormat::J2K(enc) => opj_j2k_write_tile(
              enc,
              p_tile_index,
              p_data,
              p_data_size,
              l_stream,
              &mut self.m_event_mgr,
            ),
            CodecFormat::JP2(enc) => opj_jp2_write_tile(
              enc,
              p_tile_index,
              p_data,
              p_data_size,
              l_stream,
              &mut self.m_event_mgr,
            ),
          };
        }
      }
      Codec::Decoder(_) => (),
    }
    0i32
  }
}