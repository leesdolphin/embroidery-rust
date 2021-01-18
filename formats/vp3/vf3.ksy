meta:
  id: vf3
  file-extension: vf3
  endian: be
seq:
  - id: magic
    contents: ["%vsm%", 0x00]
  - id: vendor_string
    type: wide_string
  - id: type_magic
    size: 3
  - id: bytes_remaining
    type: u4
  - id: inner
    size: bytes_remaining
    type:
      switch-on: type_magic
      cases:
        "[0x00, 0x1D, 0x00]": font_wrapper
        "[0x00, 0x02, 0x00]": pattern_wrapper
types:
  font_wrapper:
    seq:
      - id: font_name
        type: wide_string
      - id: font_character_set
        type: narrow_string
      - id: font_size
        type: u2
      - id: font_style
        type: u1
        enum: font_style
      - id: unknown_random
        size: 16
      - id: font_bound_width
        type: u2
      - id: font_bound_height
        type: u2
      - id: character_set
        type: character_set
      - id: pattern
        type: character_pattern
        repeat: expr
        repeat-expr: character_set.count
    types:
      character_set:
        seq:
          - id: count
            type: u2
          - id: characters
            type: character_set_char
            repeat: expr
            repeat-expr: count
      character_set_char:
        seq:
          - id: char
            type: str
            size: 2
            encoding: UTF-16BE
          - id: offset
            type: u4

      character_pattern:
        seq:
          - id: magic
            contents: [0, 0x11, 0]
          - id: length
            type: u4
          - id: inner
            type: pattern_inner
            size: length

  pattern_wrapper:
    seq: []

  pattern_inner:
    seq:
      - id: unknown
        size: 51
      - id: settings
        type: wide_string
      - id: unknown2
        size: 24
      - id: produced_by
        type: wide_string
      - id: thread_count
        type: u2
      - id: threads
        type: thread_wrapper
        repeat: expr
        repeat-expr: thread_count
  thread_wrapper:
    seq:
      - id: color_magic
        contents: [0, 0x05, 0]
      - id: thread_len
        type: u4
      - id: thread
        type: thread
        size: thread_len
  thread:
    seq:
      - id: start_x
        type: s4
      - id: start_y
        type: s4
      - id: table_len
        type: u1
      - id: color
        size: 3
      - id: thread_table
        size: table_len * 6
      - id: thread_num
        type: narrow_u2_string
      - id: thread_name
        type: narrow_u2_string
      - id: thread_brand
        type: narrow_u2_string
      - id: next_color_offset_x
        type: s4
      - id: next_color_offset_y
        type: s4
      - id: unknown_len
        type: u2
      - id: unknown
        size: unknown_len
      - id: colour_bytes
        type: u4
      - id: stitches
        type: stitches
        size: colour_bytes

  stitches:
    seq:
      - id: unknown2
        size: 3
      - id: stitches
        type: stitch
        repeat: eos
    types:
      stitch:
        seq:
          # Note the X and Y coordinates are 2s compliment unless equal to 0x80,
          #  then they are 0x80
          - id: x
            type: u1
          - id: y
            type: u1
          # Skip stitch if x == 0x80 and y != 0x01
          - id: ext
            type: stitch_ext
            if: x == 0x80 and y == 0x01
      stitch_ext:
        seq:
          # Note the X and Y coordinates are 2s compliment unless equal to 0x8000,
          #  then they are 0x8000
          - id: x
            type: u2
          - id: y
            type: u2
          - id: trailer
            contents: [0x80, 0x02]

  wide_string:
    seq:
      - id: len
        type: u2
      - id: str
        type: str
        size: len
        encoding: UTF-16BE
  narrow_u2_string:
    seq:
      - id: len
        type: u2
      - id: str
        type: str
        size: len
        encoding: UTF-8
  narrow_string:
    seq:
      - id: len
        type: u1
      - id: str
        type: str
        size: len
        encoding: UTF-8

enums:
  font_style:
    0x00: normal
    0x01: bold
    0x02: italic
    0x03: bold_italic
