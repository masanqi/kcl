# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: modfile/modfile.proto
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x15modfile/modfile.proto\x12\rkclvm.modfile\"\xa2\x01\n\nKclModFile\x12\x0c\n\x04root\x18\x01 \x01(\t\x12\x10\n\x08root_pkg\x18\x02 \x01(\t\x12\x36\n\x05\x62uild\x18\x03 \x01(\x0b\x32\'.kclvm.modfile.KclModFile_build_section\x12<\n\x08\x65xpected\x18\x04 \x01(\x0b\x32*.kclvm.modfile.KclModFile_expected_section\"_\n\x18KclModFile_build_section\x12\x18\n\x10\x65nable_pkg_cache\x18\x01 \x01(\x08\x12\x19\n\x11\x63\x61\x63hed_pkg_prefix\x18\x02 \x01(\t\x12\x0e\n\x06target\x18\x03 \x01(\t\"\x98\x01\n\x1bKclModFile_expected_section\x12\x16\n\x0emin_build_time\x18\x01 \x01(\t\x12\x16\n\x0emax_build_time\x18\x02 \x01(\t\x12\x15\n\rkclvm_version\x18\x03 \x01(\t\x12\x1a\n\x12kcl_plugin_version\x18\x04 \x01(\t\x12\x16\n\x0eglobal_version\x18\x05 \x01(\tb\x06proto3')



_KCLMODFILE = DESCRIPTOR.message_types_by_name['KclModFile']
_KCLMODFILE_BUILD_SECTION = DESCRIPTOR.message_types_by_name['KclModFile_build_section']
_KCLMODFILE_EXPECTED_SECTION = DESCRIPTOR.message_types_by_name['KclModFile_expected_section']
KclModFile = _reflection.GeneratedProtocolMessageType('KclModFile', (_message.Message,), {
  'DESCRIPTOR' : _KCLMODFILE,
  '__module__' : 'modfile.modfile_pb2'
  # @@protoc_insertion_point(class_scope:kclvm.modfile.KclModFile)
  })
_sym_db.RegisterMessage(KclModFile)

KclModFile_build_section = _reflection.GeneratedProtocolMessageType('KclModFile_build_section', (_message.Message,), {
  'DESCRIPTOR' : _KCLMODFILE_BUILD_SECTION,
  '__module__' : 'modfile.modfile_pb2'
  # @@protoc_insertion_point(class_scope:kclvm.modfile.KclModFile_build_section)
  })
_sym_db.RegisterMessage(KclModFile_build_section)

KclModFile_expected_section = _reflection.GeneratedProtocolMessageType('KclModFile_expected_section', (_message.Message,), {
  'DESCRIPTOR' : _KCLMODFILE_EXPECTED_SECTION,
  '__module__' : 'modfile.modfile_pb2'
  # @@protoc_insertion_point(class_scope:kclvm.modfile.KclModFile_expected_section)
  })
_sym_db.RegisterMessage(KclModFile_expected_section)

if _descriptor._USE_C_DESCRIPTORS == False:

  DESCRIPTOR._options = None
  _KCLMODFILE._serialized_start=41
  _KCLMODFILE._serialized_end=203
  _KCLMODFILE_BUILD_SECTION._serialized_start=205
  _KCLMODFILE_BUILD_SECTION._serialized_end=300
  _KCLMODFILE_EXPECTED_SECTION._serialized_start=303
  _KCLMODFILE_EXPECTED_SECTION._serialized_end=455
# @@protoc_insertion_point(module_scope)