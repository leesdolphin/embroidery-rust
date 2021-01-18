meta:
  id: vp4
  file-extension: vp4
  endian: le
  license: CC0-1.0
  ks-version: 0.8
seq:
  - type: file_header
  - type: hoop
types:
  file_header:
    seq:
      - id: magic
        contents: ["%Vp4%"]
      - id: unknown1
        size: 0x18
      - contents: "info"
      - id: info
        size: 0x08
      - contents: "nttn"
      - id: nttn
        size: 10
      - contents: "ntes"
      - id: ntes
        size: 2
      - contents: "stgs"
        id: stgs
        size: 15
  hoop:
    seq:
      - contents: "hoop"
